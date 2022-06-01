use crate::sources::{spotify, youtube};
use chrono::Duration;
use rand::Rng;
use rspotify::model::PlayableItem;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Colour,
};
use songbird::{tracks::TrackQueue, Call};

//enqueues the source of the track found on youtube and returns the full
pub async fn queueTrack<'a>(
    ctx: &Context,
    msg: &Message,
    trackName: &str,
    handler: &'a mut Call,
) -> CommandResult<Option<&'a TrackQueue>> {
    let source = match youtube::getSource(&ctx, &msg, &trackName).await? {
        Some(source) => source,
        None => {
            return Ok(None);
        }
    };

    if let Some(name) = &source.metadata.title {
        let duration = match &source.metadata.duration {
            Some(duration) => Duration::from_std(duration.to_owned()).unwrap(),
            None => Duration::zero(),
        };
        let url = &source.metadata.source_url.to_owned().unwrap();
        let thumbnial = &source.metadata.thumbnail.to_owned().unwrap();
        let artist = &source.metadata.artist.to_owned().unwrap();
        let author = &msg.author.name;

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(name)
                        .description("🎙️ Se ha añadido a la lista de canciones")
                        .fields(vec![
                            ("Autor: ", artist, true),
                            ("Solicitado por:", author, true),
                            (
                                "⌚ Duracion:",
                                &format!("{} minutes", Duration::num_minutes(&duration)),
                                true,
                            ),
                        ])
                        .url(url)
                        .thumbnail(thumbnial)
                        .colour(Colour::from_rgb(
                            rand::thread_rng().gen_range(0..255),
                            rand::thread_rng().gen_range(0..255),
                            rand::thread_rng().gen_range(0..255),
                        ))
                });

                m
            })
            .await
            .expect("Coudln't send the message");
    }

    handler.enqueue_source(source);

    Ok(Some(handler.queue()))
}

pub async fn queuePlayList<'a>(
    ctx: &Context,
    msg: &Message,
    playListName: &str,
    handler: &'a mut Call,
) -> CommandResult<Option<&'a TrackQueue>> {
    let playListResult = spotify::getPlayList(ctx, msg, playListName).await?;

    for track in playListResult.as_ref().unwrap().tracks.items.iter() {
        match track.track.as_ref().unwrap() {
            PlayableItem::Track(t) => {
                let source = match youtube::getSource(&ctx, &msg, &t.name[..]).await? {
                    Some(source) => source,
                    None => {
                        return Ok(None);
                    }
                };

                handler.enqueue_source(source);
            }
            _ => {
                return Ok(None);
            }
        }
    }

    Ok(Some(handler.queue()))
}

// shows the list of track that are in the track queue
pub async fn showQueueList(ctx: &Context, msg: &Message) -> CommandResult {
    let mut queueList: Vec<(String, String, bool)> = Vec::new();
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let mut i = 0;

    let trackQueue = handler.queue(); 

    //iterate over the lists of tracks
    for track in trackQueue.current_queue().iter() {
        if let Some(trackName) = track.metadata().title.to_owned() {
            queueList.push((format!("💿 {}.", i + 1), trackName, false))
        }

        i += 1
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            if !queueList.is_empty() {
                m.embed(|e| {
                    e.fields(queueList).colour(Colour::from_rgb(
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                    ))
                })
            } else {
                m.content("⛔ No hay mas canciones en la lista...")
            }
        })
        .await
        .expect("Coudln't send the message");

    Ok(())
}
