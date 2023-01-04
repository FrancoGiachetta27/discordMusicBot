use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::tracks::TrackQueue;
use std::time::Duration;

use crate::bot::queue;
use crate::utils;

// makes the bot join the channel where the message's author is, if not in any channel it won't work
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap(); // gets an instance of the server where the bot is in
    let guild_id = guild.id;
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id); //gets the channel id where the message's author is

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.channel_id
                .say(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let manager = songbird::get(&ctx).await.unwrap().clone(); //creates a voice client
    if let (_handler, Err(why)) = manager.join(guild_id, connect_to).await {
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
    track_name: Option<String>,
    play_list_name: Option<&str>,
) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    let track_queue: &TrackQueue = match track_name {
        Some(track) => match queue::queue_track(ctx, msg, track, &mut handler).await? {
            Some(queue) => queue,
            None => {
                return Ok(());
            }
        },
        None => {
            match queue::queue_play_list(ctx, msg, play_list_name.unwrap(), &mut handler).await? {
                Some(queue) => queue,
                None => {
                    return Ok(());
                }
            }
        }
    };

    if let Some(current) = track_queue.current() {
        current.play()?;
    };

    Ok(())
}

//stop the track permanently
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    track_queue.stop(); // stops the track and deletes the trackQueue

    Ok(())
}

//pauses the current track
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    track_queue.pause()?;

    Ok(())
}

//Unpauses a the current track
pub async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    match track_queue.current() {
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
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    track_queue.skip()?;

    track_queue.modify_queue(|queue| {
        queue.remove(0); // remove the skipped track
    });

    if let Some(new_track) = track_queue.current() {
        new_track.play()?;
    }

    Ok(())
}

pub async fn to_loop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    if let Some(track) = track_queue.current() {
        match track.enable_loop() {
            Ok(()) => {
                utils::send_message_single_line("🔁 Loop habilitado", "-", true, ctx, msg).await;
            }
            Err(why) => {
                println!("Err {}", why);
                utils::send_message_single_line(
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

pub async fn end_loop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    if let Some(track) = track_queue.current() {
        match track.disable_loop() {
            Ok(()) => {
                utils::send_message_single_line("🛑 Loop deshabilitado", "-", true, ctx, msg).await;
            }
            Err(why) => {
                println!("Err {}", why);

                return Ok(());
            }
        }
    }

    Ok(())
}

pub async fn seek(ctx: &Context, msg: &Message, time: Duration) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handler_lock = match manager.get(guild_id) {
        Some(handler_lock) => handler_lock,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(());
        }
    };

    let handler = handler_lock.lock().await;

    let track_queue = handler.queue();

    if let Some(track) = track_queue.current() {
        match track.seek_time(time) {
            Ok(()) => {}
            Err(_err) => {
                utils::send_message_single_line(
                    "❌ Limite de tiempo sobrepasado",
                    "-",
                    true,
                    ctx,
                    msg,
                )
                .await;
            }
        };
    }

    Ok(())
}

