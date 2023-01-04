use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use songbird::tracks::TrackQueue;
use std::time::Duration;

use crate::bot::{music_bot, queue};
use crate::lyrics::genius_lyrics::get_lyrics;
use crate::utils;

#[group]
#[commands(
    play, pause, resume, stop, skip, seek, toloop, endloop, /*config*/ queue, /*playlist*/ lyrics
)]
struct General;

// functions which are called when a command is sent. For example: -play....
#[command]
#[aliases("p")]
#[usage = "⏯️  <prefix>p"]
#[description = "reproducir canciones"]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
    let track = args.raw().collect::<Vec<&str>>().join(" "); // gets the arguments passed, in tis case the song's name

    music_bot::join(&ctx, &msg).await?;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let track_queue: &TrackQueue =
            match queue::queue_track(ctx, msg, track, &mut handler).await? {
                Some(queue) => queue,
                None => {
                    return Ok(());
                }
            };

        if let Some(current) = track_queue.current() {
            current.play()?;
        };
    }

    Ok(())
}

#[command]
#[usage = "🛑 <prefix>pause"]
#[description = "pausar una cancion"]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        track_queue.pause()?;
    }

    Ok(())
}

#[command]
#[usage = "⏯️  <prefix>resume"]
#[description = "frenar definitivamente una cancion"]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
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
    }

    Ok(())
}

#[command]
#[usage = "🛑 <prefix>stop"]
#[description = "reanudar una cancion pausada"]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        track_queue.stop(); // stops the track and deletes the trackQueue
    }

    Ok(())
}

#[command]
#[usage = "⏭️  <prefix>skip"]
#[description = "saltear una cacion"]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        track_queue.skip()?;

        track_queue.modify_queue(|queue| {
            queue.remove(0); // remove the skipped track
        });

        if let Some(new_track) = track_queue.current() {
            new_track.play()?;
        }
    }

    Ok(())
}

#[command]
#[usage = "🔍 <prefix>seek {num} {-s / -m / -h} "]
#[description = "saltar a un segundo/minuto/hora deseado (si es posible)"]
async fn seek(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let content: Vec<&str> = args.raw().collect::<Vec<&str>>();
        let mut time: Duration = Duration::new(0, 0);

        match content[1] {
            "-s" => {
                time = Duration::from_secs_f64(content[0].parse::<f64>().unwrap());

                utils::send_message_single_line(
                    &format!("🦘 Salto al segundo {:?}", &time)[..],
                    "-",
                    false,
                    ctx,
                    msg,
                )
                .await;
            }
            "-m" => {
                time = Duration::from_secs_f64(content[0].parse::<f64>().unwrap() * 60.0);

                utils::send_message_single_line(
                    &format!("🦘 Salto al minuto {:?}", &time)[..],
                    "-",
                    false,
                    ctx,
                    msg,
                )
                .await;
            }
            "-h" => {
                time = Duration::from_secs_f64(content[0].parse::<f64>().unwrap() * 360.0);

                utils::send_message_single_line(
                    &format!("🦘 Salto a la hora {:?}", &time)[..],
                    "-",
                    false,
                    ctx,
                    msg,
                )
                .await;
            }
            _ => {
                utils::send_message_single_line("🛑 Comando incorrecto", "-", false, ctx, msg)
                    .await;
            }
        }
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
    }

    Ok(())
}

#[command]
#[usage = "♾️ <prefix>toloop"]
#[description = "repetir la cancion infinitamente"]
async fn toloop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        if let Some(track) = track_queue.current() {
            match track.enable_loop() {
                Ok(()) => {
                    utils::send_message_single_line("🔁 Loop habilitado", "-", true, ctx, msg)
                        .await;
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
    }

    Ok(())
}

#[command]
#[usage = "🔁 <prefix>endloop"]
#[description = "frenar la repeticion"]
async fn endloop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        if let Some(track) = track_queue.current() {
            match track.disable_loop() {
                Ok(()) => {
                    utils::send_message_single_line("🛑 Loop deshabilitado", "-", true, ctx, msg)
                        .await;
                }
                Err(why) => {
                    println!("Err {}", why);

                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

#[command]
#[usage = "<prefix>queue"]
#[description = ""]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        queue::show_queue_list(ctx, msg, &mut handler).await?;
    }

    Ok(())
}

// #[command]
// #[usage = "⏯️  <prefix>playlist"]
// #[description = "reproducir una playlist de spotify"]
// async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;
//     let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
//     let play_list_name: Vec<&str> = utils::get_track_name(&msg.content[..]);
//
//     music_bot::join(&ctx, &msg).await?;
//
//     if let Some(handler_lock) = manager.get(guild_id) {
//         let handler = handler_lock.lock().await;
//     }
//     Ok(())
// }

#[command]
#[usage = "📜  <prefix>lyrics"]
#[description = "obtener la letra de la cancion que se esta reproducioendo"]
async fn lyrics(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let track_queue = handler.queue();

        if let Some(track) = track_queue.current() {
            get_lyrics(
                &ctx,
                &msg,
                &mut track.metadata().title.as_ref().unwrap().as_str(),
            )
            .await?;
        }
    }

    Ok(())
}
