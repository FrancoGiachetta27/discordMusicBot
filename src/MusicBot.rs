use rand::Rng;
use serenity::{
    client::Context, framework::standard::CommandResult, model::channel::Message, utils::Colour,
};
use songbird::tracks::{TrackHandle, TrackQueue, TrackState};

use crate::queue;

//play a track
pub async fn play(
    ctx: &Context,
    msg: &Message,
    trackName: Option<&str>,
    playList: Option<&str>,
) -> CommandResult {
    let mut break_ = false;
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
    let mut break_: bool = false;

    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue: &TrackQueue =
        match queue::queue(ctx, msg, trackName, playList, &mut handler).await? {
            Some(queue) => queue,
            None => {
                return Ok(());
            }
        };

    let mut currentTrack: Option<TrackHandle> = trackQueue.current();

    let trackStatus: Option<TrackState> = if let Some(currentTrack) = &currentTrack {
        Some(currentTrack.get_info().await?)
    } else {
        return Ok(());
    };

    while !trackQueue.is_empty() {
        if let Some(currentTrack) = &currentTrack {
            currentTrack.play()?;
        }

        break_ = msg.content[..].starts_with("-p")
            || msg.content[..].starts_with("-pause")
            || msg.content[..].starts_with("-skip")
            || msg.content[..].starts_with("-stop");

        if break_ {
            break;
        }
    }

    Ok(())
}

//stop the track permanently
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.stop(); // stops the track and deletes the trackQueue

    Ok(())
}

//pauses the current track
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.pause()?;

    Ok(())
}

//Unpauses a the current track
pub async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    match trackQueue.current() {
        Some(track) => {
            track.play()?;
        }
        None => {
            msg.reply(&ctx.http, "❌ | No hay mas canciones para reproducir")
                .await?;
        }
    }

    Ok(())
}

//skips the current track
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.skip()?;

    trackQueue.modify_queue(|queue| {
        queue.remove(0); // remove the skipped track
    });

    if let Some(newTrack) = trackQueue.current() {
        newTrack.play()?;
    }

    Ok(())
}

pub async fn toLoop(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue: &TrackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    let currentTrack = trackQueue.current();

    if let Some(track) = currentTrack {
        match track.enable_loop() {
            Ok(ok) => {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.field("🔁 Loop habilitado", "", true)
                                .colour(Colour::from_rgb(
                                    rand::thread_rng().gen_range(0..255),
                                    rand::thread_rng().gen_range(0..255),
                                    rand::thread_rng().gen_range(0..255),
                                ))
                        })
                    })
                    .await
                    .expect("Coudln't send the message");
            }
            Err(why) => {
                println!("Err {}", why);
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.field(
                                "❌ Esta cancion no tiene la opcion 'loop' habilitada",
                                "",
                                true,
                            )
                            .colour(Colour::from_rgb(
                                rand::thread_rng().gen_range(0..255),
                                rand::thread_rng().gen_range(0..255),
                                rand::thread_rng().gen_range(0..255),
                            ))
                        })
                    })
                    .await
                    .expect("Coudln't send the message");

                return Ok(());
            }
        }
    }

    Ok(())
}

pub async fn endLoop(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue: &TrackQueue = match queue::queue(ctx, msg, None, None, &mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    let currentTrack = trackQueue.current();

    if let Some(track) = currentTrack {
        match track.disable_loop() {
            Ok(ok) => {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.field("🛑 Loop deshabilitado", "", true)
                                .colour(Colour::from_rgb(
                                    rand::thread_rng().gen_range(0..255),
                                    rand::thread_rng().gen_range(0..255),
                                    rand::thread_rng().gen_range(0..255),
                                ))
                        })
                    })
                    .await
                    .expect("Coudln't send the message");
            }
            Err(why) => {
                println!("Err {}", why);

                return Ok(());
            }
        }
    }

    Ok(())
}