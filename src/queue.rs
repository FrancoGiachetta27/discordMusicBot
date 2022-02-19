use crate::{playlist, spotify, youtube};
use chrono::Duration;
use rand::Rng;
use rspotify::model::PlayableItem;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Colour,
};
use songbird::{tracks::TrackQueue, Call};

//enqueues the source of the track found on youtube and returns the full
pub async fn queue<'a>(
    ctx: &Context,
    msg: &Message,
    trackName: Option<&str>,
    playListName: Option<&str>,
    handler: &'a mut Call,
) -> CommandResult<Option<&'a TrackQueue>> {
    if let Some(trackName) = trackName {
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

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.fields(vec![
                            (
                                "🎙️ Se ha añadido una cancion a la lista de canciones:",
                                name,
                                false,
                            ),
                            ("Solicitado por:", &msg.author.name, true),
                            (
                                "⌚ Duracion:",
                                &format!("{} minutes", Duration::num_minutes(&duration)),
                                true,
                            ),
                        ])
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
    } else if let Some(name) = playListName {
        let playListResult = spotify::getPlayList(ctx, msg, name).await?;

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
    }

    Ok(Some(handler.queue()))
}

// shows the list of track that are in the track qeue
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

    let trackQueue: &TrackQueue = match queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    let mut i = 0;

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
