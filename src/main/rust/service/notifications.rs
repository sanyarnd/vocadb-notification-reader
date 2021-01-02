use std::cmp::Ordering;

use regex::Regex;

use crate::client::client::Client;
use crate::client::models::query::LanguagePreference;
use crate::client::models::user::UserMessageContract;
use crate::errors::AppError;
use crate::service::auth::Database;
use crate::service::dto::{
    Album, Artist, Event, Notification, Report, Song, SongNotificationType, Tag, Unknown, PV,
};

pub async fn load_notification_details<'a>(
    client: &Client<'a>,
    database: &Database,
    language: &LanguagePreference,
    message_id: i32,
) -> Result<Box<dyn Notification>, AppError> {
    let message = client.get_message(message_id).await?;
    let body = &message.body;

    let re = Regex::new(&format!(r"https?://{}/(\w+)/(\d+)", database.domain()))?;

    let captures = re.captures(body);

    return Ok(match captures {
        None => unknown_notification(&message),
        Some(captures) => {
            let notification_type = captures.get(1).map(|t| t.as_str());
            let id = captures.get(2).map(|id| id.as_str());

            match notification_type {
                None => unknown_notification(&message),
                Some(n_type) => match n_type {
                    "S" => {
                        let song_id = id.unwrap().parse::<i32>()?;
                        song_notification(client, song_id, &message, language).await?
                    }
                    "Ar" => artist_notification(&message),
                    "E" => event_notification(&message),
                    "Al" => album_notification(&message),
                    "R" => report_notification(&message),
                    _ => unknown_notification(&message),
                },
            }
        }
    });
}

fn report_notification(message: &UserMessageContract) -> Box<Report> {
    Box::new(Report {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
    })
}

fn album_notification(message: &UserMessageContract) -> Box<Album> {
    Box::new(Album {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
    })
}

fn event_notification(message: &UserMessageContract) -> Box<Event> {
    Box::new(Event {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
    })
}

fn artist_notification(message: &UserMessageContract) -> Box<Artist> {
    Box::new(Artist {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
    })
}

async fn song_notification<'a>(
    client: &Client<'a>,
    song_id: i32,
    message: &UserMessageContract,
    language: &LanguagePreference,
) -> Result<Box<Song>, AppError> {
    let song = client.get_song_by_id(song_id, language).await?;
    let song_notification_type = if message.subject.contains("tagged") {
        SongNotificationType::Tagged
    } else {
        SongNotificationType::New
    };
    let artist = song.artist_string;
    let title = song.name;
    let publish_date = song.publish_date;
    let song_type = song.song_type;
    let mut tags: Vec<Tag> = song
        .tags
        .iter()
        .map(|t| Tag {
            id: t.tag.id,
            name: t.tag.name.clone(),
            count: t.count,
            category_name: t.tag.category_name.clone(),
        })
        .collect();

    tags.sort_by(|a, b| match a.count.cmp(&b.count) {
        Ordering::Equal => a.name.cmp(&b.name),
        other => other,
    });

    let mut pvs: Vec<PV> = song.pvs.iter().map(PV::from).collect();
    pvs.sort_by(
        |a, b| match a.service.to_string().cmp(&b.service.to_string()) {
            Ordering::Equal => a.pv_type.to_string().cmp(&b.pv_type.to_string()),
            other => other,
        },
    );

    Ok(Box::new(Song {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
        song_notification_type,
        song_id,
        song_type,
        tags,
        pvs,
        title,
        artist,
        release_date: publish_date,
    }))
}

fn unknown_notification(message: &UserMessageContract) -> Box<dyn Notification> {
    return Box::new(Unknown {
        id: message.id,
        subject: message.subject.clone(),
        body: message.body.clone(),
        created_date: message.created_formatted.clone(),
    });
}
