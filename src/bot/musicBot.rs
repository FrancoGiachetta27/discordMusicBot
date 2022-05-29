use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::tracks::{
    PlayMode,
    TrackHandle, TrackQueue, TrackState
};

use crate::utils;

use crate::queue;

//play a track
pub async fn play(
    ctx: &Context,
    msg: &Message,
    trackName: Option<&str>,
    playList: Option<&str>,
) -> CommandResult {
    let mut breakable: bool = false;

    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
        if trackStatus.unwrap().playing != PlayMode::Play {
           match trackQueue.dequeue(0) {
               Some(track) => {
                   track.handle().play()?;
               }
               None => {
                    msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                    .await?;

                    return Ok(());
               }
           }  
        }

        breakable = ctx.cache.channel_messages_field(msg.channel_id, |msgs| {
            let lastMessage = utils::MessageToVector(&msgs.last().unwrap().content[..]);

            //checks whether the users have requested another song 
            lastMessage[1] != trackName.unwrap() && lastMessage[0] == "-p" && 
            lastMessage[0] == "-play" && lastMessage[0] == "-stop" &&
            lastMessage[0] == "-pause" && lastMessage[0] == "-skip" && lastMessage[0] == "-resume" && 
            lastMessage[0] == "-toloop" && lastMessage[0] == "-endloop" && lastMessage[0] == "-help" && 
            lastMessage[0] == "-lyrics" && lastMessage[0] == "-playlist"
        }).await.unwrap();

        if breakable {
            break;
        }
    }

    Ok(())
}

//stop the track permanently
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
                utils::sendMessageSingleLine("🔁 Loop habilitado", "", true, ctx, msg).await;
            }
            Err(why) => {
                println!("Err {}", why);
                utils::sendMessageSingleLine(
                    "❌ Esta cancion no tiene la opcion 'loop' habilitada",
                    "",
                    true,
                    ctx,
                    msg,
                )
                .await;

                return Ok(());
            }
        }
    }

    Ok(())
}

pub async fn endLoop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut handler = match utils::gethandler(ctx, msg).await.unwrap() {
        Some(handler) => handler.lock().await.clone(),
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

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
                utils::sendMessageSingleLine("🛑 Loop deshabilitado", "", true, ctx, msg).await;
            }
            Err(why) => {
                println!("Err {}", why);

                return Ok(());
            }
        }
    }

    Ok(())
}
