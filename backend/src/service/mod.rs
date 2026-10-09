//! Converts raw VocaDB inbox messages into typed notifications.

pub mod dto;

use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use futures::{StreamExt, TryStreamExt};

use crate::cache::{Cache, INBOX_TTL, MESSAGE_TTL, SONG_TTL};
use crate::client::models::{LanguagePreference, SongForApiContract, UserMessageContract};
use crate::client::{Client, Result};
use crate::service::dto::{
    BaseNotification, Notification, PV, SongNotification, SongNotificationType, Tag,
};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Debug, TS)]
#[ts(export)]
#[allow(clippy::enum_variant_names)]
pub enum Database {
    VocaDb,
    TouhouDb,
    UtaiteDb,
}

impl Database {
    pub const ALL: [Database; 3] = [Database::VocaDb, Database::TouhouDb, Database::UtaiteDb];

    pub fn domain(self) -> &'static str {
        match self {
            Database::VocaDb => "vocadb.net",
            Database::TouhouDb => "touhoudb.com",
            Database::UtaiteDb => "utaitedb.net",
        }
    }

    pub fn default_url(self) -> String {
        format!("https://{}", self.domain())
    }

    fn entry_link_regex(self) -> &'static Regex {
        fn build(domain: &str) -> Regex {
            Regex::new(&format!(r"https?://{}/(\w+)/(\d+)", regex::escape(domain)))
                .expect("entry link regex must be valid")
        }

        static VOCADB: LazyLock<Regex> = LazyLock::new(|| build(Database::VocaDb.domain()));
        static TOUHOUDB: LazyLock<Regex> = LazyLock::new(|| build(Database::TouhouDb.domain()));
        static UTAITEDB: LazyLock<Regex> = LazyLock::new(|| build(Database::UtaiteDb.domain()));

        match self {
            Database::VocaDb => &VOCADB,
            Database::TouhouDb => &TOUHOUDB,
            Database::UtaiteDb => &UTAITEDB,
        }
    }
}

/// Type of a notification, as shown by the frontend tabs.
#[derive(Serialize, Deserialize, TS, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum NotificationKind {
    Song,
    Artist,
    Album,
    Event,
    Report,
    Unknown,
}

/// Amount of notifications of every kind.
#[derive(Serialize, TS, Debug, Default, Clone, PartialEq, Eq)]
#[ts(export)]
pub struct KindCounts {
    pub song: u32,
    pub artist: u32,
    pub album: u32,
    pub event: u32,
    pub report: u32,
    pub unknown: u32,
}

impl KindCounts {
    pub fn add(&mut self, kind: NotificationKind) {
        let counter = match kind {
            NotificationKind::Song => &mut self.song,
            NotificationKind::Artist => &mut self.artist,
            NotificationKind::Album => &mut self.album,
            NotificationKind::Event => &mut self.event,
            NotificationKind::Report => &mut self.report,
            NotificationKind::Unknown => &mut self.unknown,
        };
        *counter += 1;
    }
}

/// Kind of a notification, derived from a message subject and body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Song(i32),
    Artist,
    Album,
    Event,
    Report,
    Unknown,
}

impl MessageKind {
    pub fn kind(self) -> NotificationKind {
        match self {
            MessageKind::Song(_) => NotificationKind::Song,
            MessageKind::Artist => NotificationKind::Artist,
            MessageKind::Album => NotificationKind::Album,
            MessageKind::Event => NotificationKind::Event,
            MessageKind::Report => NotificationKind::Report,
            MessageKind::Unknown => NotificationKind::Unknown,
        }
    }
}

pub fn classify_message(database: Database, subject: &str, body: &str) -> MessageKind {
    let Some(captures) = database.entry_link_regex().captures(body) else {
        return MessageKind::Unknown;
    };

    if subject.contains("Entry report") {
        return MessageKind::Report;
    }

    match &captures[1] {
        "S" => captures[2]
            .parse()
            .map_or(MessageKind::Unknown, MessageKind::Song),
        "Ar" => MessageKind::Artist,
        "Al" => MessageKind::Album,
        "E" => MessageKind::Event,
        _ => MessageKind::Unknown,
    }
}

/// Where notifications are loaded from.
pub struct Source<'a> {
    pub client: &'a Client,
    pub cache: &'a Cache,
    pub database: Database,
    pub user_id: i32,
}

/// Upper bound of messages read from an inbox.
pub const MAX_INBOX_SIZE: usize = 5000;
/// Messages requested per page of the inbox listing.
const LIST_PAGE_SIZE: i32 = 50;
/// Simultaneous message requests while reading the inbox; messages are cached afterwards.
const INBOX_CONCURRENCY: usize = 16;

/// A message of the inbox together with its kind.
#[derive(Debug)]
pub struct InboxEntry {
    pub message: UserMessageContract,
    pub kind: MessageKind,
}

impl InboxEntry {
    /// Case-insensitive search in the subject and the text, which contain the names
    /// of the entries (e.g. song title and artist).
    pub fn matches(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();
        query.is_empty()
            || self.message.subject.to_lowercase().contains(&query)
            || self.message.body.to_lowercase().contains(&query)
    }
}

/// IDs of all messages in the notifications inbox, newest first.
pub async fn inbox_ids(source: &Source<'_>) -> Result<Vec<i32>> {
    let fetch = async {
        let mut ids = Vec::new();
        loop {
            let offset = i32::try_from(ids.len()).unwrap_or(i32::MAX);
            let page = source
                .client
                .get_messages(source.user_id, offset, LIST_PAGE_SIZE)
                .await?;
            let received = page.items.len();
            ids.extend(page.items.into_iter().map(|m| m.id));

            let total = usize::try_from(page.total_count).unwrap_or(0);
            if received == 0 || ids.len() >= total || ids.len() >= MAX_INBOX_SIZE {
                break;
            }
        }
        ids.truncate(MAX_INBOX_SIZE);
        Ok(serde_json::to_string(&ids).expect("IDs are serializable"))
    };

    source
        .cache
        .get_or_fetch(
            &Cache::inbox_key(source.database, source.user_id),
            INBOX_TTL,
            fetch,
        )
        .await
}

pub async fn load_message(source: &Source<'_>, message_id: i32) -> Result<UserMessageContract> {
    source
        .cache
        .get_or_fetch(
            &Cache::message_key(source.database, source.user_id, message_id),
            MESSAGE_TTL,
            source.client.get_message_json(message_id),
        )
        .await
}

/// Every message of the notifications inbox with its kind, newest first.
///
/// Every message has to be read once to tell its kind; note that VocaDB marks a message
/// as read when it is loaded.
pub async fn inbox(source: &Source<'_>) -> Result<Vec<InboxEntry>> {
    let ids = inbox_ids(source).await?;
    futures::stream::iter(ids)
        .map(|id| async move {
            let message = load_message(source, id).await?;
            let kind = classify_message(source.database, &message.subject, &message.body);
            Ok(InboxEntry { message, kind })
        })
        .buffered(INBOX_CONCURRENCY)
        .try_collect()
        .await
}

/// Turns an inbox entry into a notification, loading the song details if needed.
pub async fn notification(
    source: &Source<'_>,
    entry: InboxEntry,
    language: LanguagePreference,
) -> Result<Notification> {
    let base = base_notification(entry.message);
    Ok(match entry.kind {
        MessageKind::Song(song_id) => {
            Notification::Song(song_notification(source, song_id, base, language).await?)
        }
        MessageKind::Artist => Notification::Artist(base),
        MessageKind::Album => Notification::Album(base),
        MessageKind::Event => Notification::Event(base),
        MessageKind::Report => Notification::Report(base),
        MessageKind::Unknown => Notification::Unknown(base),
    })
}

fn base_notification(message: UserMessageContract) -> BaseNotification {
    BaseNotification {
        id: message.id,
        original_subject: message.subject,
        original_body: message.body,
        created_date: message.created_formatted,
    }
}

async fn song_notification(
    source: &Source<'_>,
    song_id: i32,
    base: BaseNotification,
    language: LanguagePreference,
) -> Result<SongNotification> {
    let song: SongForApiContract = source
        .cache
        .get_or_fetch(
            &Cache::song_key(source.database, song_id, language),
            SONG_TTL,
            source.client.get_song_json(song_id, language),
        )
        .await?;

    let song_notification_type = if base.original_subject.contains("tagged") {
        SongNotificationType::Tagged
    } else {
        SongNotificationType::New
    };

    let mut tags: Vec<Tag> = song
        .tags
        .into_iter()
        .map(|usage| Tag {
            id: usage.tag.id,
            name: usage.tag.name,
            count: usage.count,
            category_name: usage.tag.category_name,
        })
        .collect();
    tags.sort_by(|a, b| a.count.cmp(&b.count).then_with(|| a.name.cmp(&b.name)));

    let mut pvs: Vec<PV> = song.pvs.into_iter().map(PV::from).collect();
    pvs.sort_by(|a, b| {
        a.service
            .as_ref()
            .cmp(b.service.as_ref())
            .then_with(|| a.pv_type.as_ref().cmp(b.pv_type.as_ref()))
    });

    Ok(SongNotification {
        base,
        song_notification_type,
        song_id,
        song_type: song.song_type,
        tags,
        pvs,
        title: song.name,
        artist: song.artist_string,
        release_date: song.publish_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_messages_by_link() {
        let cases = [
            (
                "New song",
                "https://vocadb.net/S/123",
                MessageKind::Song(123),
            ),
            (
                "New artist",
                "see http://vocadb.net/Ar/1 now",
                MessageKind::Artist,
            ),
            ("New album", "https://vocadb.net/Al/1", MessageKind::Album),
            ("New event", "https://vocadb.net/E/1", MessageKind::Event),
            (
                "Entry report",
                "https://vocadb.net/S/1",
                MessageKind::Report,
            ),
            ("Something", "https://vocadb.net/T/1", MessageKind::Unknown),
            ("Something", "no links here", MessageKind::Unknown),
        ];

        for (subject, body, expected) in cases {
            assert_eq!(
                classify_message(Database::VocaDb, subject, body),
                expected,
                "{body}"
            );
        }
    }

    #[test]
    fn ignores_links_from_other_databases() {
        assert_eq!(
            classify_message(Database::TouhouDb, "New song", "https://vocadb.net/S/1"),
            MessageKind::Unknown
        );
        assert_eq!(
            classify_message(Database::UtaiteDb, "New song", "https://utaitedb.net/S/7"),
            MessageKind::Song(7)
        );
    }

    #[test]
    fn domain_dots_are_not_wildcards() {
        assert_eq!(
            classify_message(Database::VocaDb, "New song", "https://vocadbxnet/S/1"),
            MessageKind::Unknown
        );
    }

    fn entry(subject: &str, body: &str) -> InboxEntry {
        let message = serde_json::from_value(serde_json::json!({
            "id": 1,
            "subject": subject,
            "body": body,
            "createdFormatted": "11.02.2018 11:19"
        }))
        .unwrap();
        InboxEntry {
            message,
            kind: MessageKind::Unknown,
        }
    }

    #[test]
    fn searches_subject_and_text() {
        let entry = entry(
            "New song tagged with VOCALOID",
            "A new song, '[Melt](https://vocadb.net/S/1)', by ryo was just added.",
        );
        assert!(entry.matches(""));
        assert!(entry.matches("  "));
        assert!(entry.matches("vocaloid"));
        assert!(entry.matches(" MELT "));
        assert!(entry.matches("ryo"));
        assert!(!entry.matches("rin"));
    }

    #[test]
    fn counts_kinds() {
        let mut counts = KindCounts::default();
        for kind in [
            MessageKind::Song(1),
            MessageKind::Song(2),
            MessageKind::Report,
        ] {
            counts.add(kind.kind());
        }
        assert_eq!(
            counts,
            KindCounts {
                song: 2,
                report: 1,
                ..KindCounts::default()
            }
        );
    }
}
