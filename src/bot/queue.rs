use crate::sources::{spotify, youtube};
use chrono::Duration;
use rand::Rng;
use rspotify::model::PlayableItem;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Colour,
};
use songbird::{tracks::TrackQueue, Call, input::{Input, Restartable}};

//enqueues the source of the track found on youtube and returns the full queue
pub async fn queue_track<'a>(
    ctx: &Context,
    msg: &Message,
    track_name: String,
    handler: &'a mut Call,
) -> CommandResult<Option<&'a TrackQueue>> {
    let mut source = match youtube::get_source(&ctx, &msg, track_name).await? {
        Some(src) => {
            src.into()
        },
        None => {
            return Ok(None);
        }
    };

    source = show_track_info(ctx, msg, source).await?;

    handler.enqueue_source(source.into());

    Ok(Some(handler.queue()))
}

//enqueues the full playlist found on spotify and returns the full queue
pub async fn queue_play_list<'a>(
    ctx: &Context,
    msg: &Message,
    play_list_name: &str,
    handler: &'a mut Call,
) -> CommandResult<Option<&'a TrackQueue>> {
    let play_list_result = spotify::get_play_list(ctx, msg, play_list_name).await?;

    for track in play_list_result.as_ref().unwrap().tracks.items.iter() {
        match track.track.as_ref().unwrap() {
            PlayableItem::Track(t) => {
                let source = match youtube::get_source(&ctx, &msg, t.name.to_owned()).await? {
                    Some(source) => source,
                    None => {
                        return Ok(None);
                    }
                };

                handler.enqueue_source(source.into());
            }
            _ => {
                return Ok(None);
            }
        }
    }

    Ok(Some(handler.queue()))
}

pub async fn show_track_info(ctx: &Context, msg: &Message, source: Input) -> CommandResult<Input> {
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

    Ok(source)
}

// shows the list of track that are in the track queue
pub async fn show_queue_list(ctx: &Context, msg: &Message) -> CommandResult {
    let mut queue_list: Vec<(String, String, bool)> = Vec::new();
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let mut i = 0;

    let track_queue = handler.queue();

    //iterate over the lists of tracks
    for track in track_queue.current_queue().iter() {
        if let Some(track_name) = track.metadata().title.to_owned() {
            queue_list.push((format!("💿 {}.", i + 1), track_name, false))
        }

        i += 1
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            if !queue_list.is_empty() {
                m.embed(|e| {
                    e.fields(queue_list).colour(Colour::from_rgb(
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
