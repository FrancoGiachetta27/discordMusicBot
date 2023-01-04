use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use std::time::Duration;

use crate::bot::{music_bot, queue};
use crate::lyrics::genius_lyrics::get_lyrics;
use crate::utils;

#[group]
#[commands(
    play, pause, resume, stop, skip, seek, toloop, endloop, help, /*config*/ queue, playlist, lyrics
)]
struct General;

// functions which are called when a command is sent. For example: -play....
#[command]
#[aliases("p")]
#[usage = "⏯️  <prefix>p"]
#[description = "reproducir canciones"]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let track_name: Vec<&str> = utils::get_track_name(&msg.content[..]);

    music_bot::join(&ctx, &msg).await?;

    if track_name.len() == 2 {
        music_bot::play(&ctx, &msg, Some(track_name[1].to_string()), None).await?;
    }

    Ok(())
}

#[command]
#[usage = "🛑 <prefix>pause"]
#[description = "pausar una cancion"]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::pause(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "⏯️  <prefix>resume"]
#[description = "frenar definitivamente una cancion"]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::resume(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "🛑 <prefix>stop"]
#[description = "reanudar una cancion pausada"]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::stop(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "⏭️  <prefix>skip"]
#[description = "saltear una cacion"]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::skip(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "🔍 <prefix>seek {num} {-s / -m} "]
#[description = "saltar a un segundo/minuto deseado"]
async fn seek(ctx: &Context, msg: &Message) -> CommandResult {
    let content: Vec<&str> = utils::message_to_command(&msg.content[..]);
    let mut time: Duration = Duration::new(0, 0);

    match content[2] {
        "-s" => {
            time = Duration::from_secs_f64(content[1].parse::<f64>().unwrap());

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
            time = Duration::from_secs_f64(content[1].parse::<f64>().unwrap() * 60.0);

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
            time = Duration::from_secs_f64(content[1].parse::<f64>().unwrap() * 360.0);

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
            utils::send_message_single_line("🛑 Comando incorrecto", "-", false, ctx, msg).await;
        }
    }

    music_bot::seek(&ctx, &msg, time).await?;

    Ok(())
}

#[command]
#[usage = "♾️ <prefix>toloop"]
#[description = "repetir la cancion infinitamente"]
async fn toloop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::to_loop(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "🔁 <prefix>endloop"]
#[description = "frenar la repeticion"]
async fn endloop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::end_loop(&ctx, &msg).await?;

    Ok(())
}

#[command]
#[usage = "<prefix>help {comando}"]
#[description = ""]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let iterator = vec![("🛑 pause:", "pausar una cancion", false)];

    utils::send_message_multi_line(iterator, ctx, msg).await;

    Ok(())
}

// #[command]
// async fn config(ctx: &Context, msg: &Message) -> CommandResult {Ok(())}

#[command]
#[usage = "<prefix>queue"]
#[description = ""]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    queue::show_queue_list(ctx, msg).await?;

    Ok(())
}

#[command]
#[usage = "⏯️  <prefix>playlist"]
#[description = "reproducir una playlist de spotify"]
async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
    let play_list_name: Vec<&str> = utils::get_track_name(&msg.content[..]);

    music_bot::join(&ctx, &msg).await?;

    if play_list_name.len() == 2 {
        music_bot::play(&ctx, &msg, None, Some(play_list_name[1])).await?;
    }
    Ok(())
}

#[command]
#[usage = "📜  <prefix>lyrics"]
#[description = "obtener la letra de la cancion que se esta reproducioendo"]
async fn lyrics(ctx: &Context, msg: &Message) -> CommandResult {
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

    let track_queue = handler.queue();

    if let Some(track) = track_queue.current() {
        get_lyrics(
            &ctx,
            &msg,
            &mut track.metadata().title.as_ref().unwrap().as_str(),
        )
        .await?;
    }
    Ok(())
}
