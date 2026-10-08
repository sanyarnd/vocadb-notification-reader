//! Converts raw VocaDB inbox messages into typed notifications.

pub mod dto;

use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::client::models::{LanguagePreference, UserMessageContract};
use crate::client::{Client, Result};
use crate::service::dto::{
    BaseNotification, Notification, PV, SongNotification, SongNotificationType, Tag,
};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

pub async fn load_notification_details(
    client: &Client,
    database: Database,
    language: LanguagePreference,
    message_id: i32,
) -> Result<Notification> {
    let message = client.get_message(message_id).await?;
    let kind = classify_message(database, &message.subject, &message.body);
    let base = base_notification(message);

    Ok(match kind {
        MessageKind::Song(song_id) => {
            Notification::Song(song_notification(client, song_id, base, language).await?)
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
    client: &Client,
    song_id: i32,
    base: BaseNotification,
    language: LanguagePreference,
) -> Result<SongNotification> {
    let song = client.get_song_by_id(song_id, language).await?;

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
}
