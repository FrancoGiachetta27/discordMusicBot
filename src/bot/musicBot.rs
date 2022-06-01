use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::tracks::{
    TrackQueue,
};

use crate::utils;
use crate::queue;

// makes the bot join the channel where the message's author is, if not in any channel it won't work
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap(); // gets an instance of the server where the bot is in
    let guildId = guild.id;
    let channelId = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voiceState| voiceState.channel_id); //gets the channel id where the message's author is

    let connectTo = match channelId {
        Some(channel) => channel,
        None => {
            msg.channel_id
                .say(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let manager = songbird::get(&ctx).await.unwrap().clone(); //creates a voice client
    if let (handler, Err(why)) = manager.join(guildId, connectTo).await {
        println!("JoinError {}", why);
    }

    Ok(())
}

// makes the bot leave a channel
// pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = &msg.guild(&ctx.cache).await.unwrap();
//     let guildId = guild.id;
//     let manager = songbird::get(&ctx).await.unwrap().clone();
//     let hasHandler = manager.get(guildId).is_some();

//     if hasHandler {
//         if let Err(why) = manager.remove(guildId).await {
//             msg.channel_id
//                 .say(&ctx.http, "❌ | Error al desconectar el bot")
//                 .await?;
//         }

//         msg.channel_id.say(&ctx.http, "Bot desconectado").await?;
//     }

//     Ok(())
// }

//play a track
pub async fn play(
    ctx: &Context,
    msg: &Message,
    trackName: Option<&str>,
    playListName: Option<&str>,
) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;
        
    let trackQueue: &TrackQueue = match trackName {
        Some(track) => {
            match queue::queueTrack(ctx, msg, trackName.unwrap(), &mut handler).await? {
                Some(queue) => queue,
                None => {
                    return Ok(());
                }
            }
        }
        None =>  {
            match queue::queuePlayList(ctx, msg, playListName.unwrap(), &mut handler).await? {
                Some(queue) => queue,
                None => {
                    return Ok(());
                }
            }
        }
    };

    if let Some(current) = trackQueue.current() {
        current.play()?;
    };
    
    Ok(())
}

//stop the track permanently
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

    trackQueue.stop(); // stops the track and deletes the trackQueue

    Ok(())
}

//pauses the current track
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

    trackQueue.pause()?;

    Ok(())
}

//Unpauses a the current track
pub async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

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
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

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
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

    if let Some(track) =  trackQueue.current() {
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
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handlerLock) => handlerLock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = handler.queue();

    if let Some(track) = trackQueue.current() {
        match track.disable_loop() {
            Ok(()) => {
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