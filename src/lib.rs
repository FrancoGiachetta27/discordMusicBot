use rand::Rng;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, guild::GuildStatus, id::ChannelId},
    prelude::*,
    utils::Colour,
};
use songbird::{
    SerenityInit,
};
use std::env;

mod bot;
mod lyrics;
mod sources;
mod utils;

use bot::{music_bot, queue};
use lyrics::genius_lyrics::get_lyrics;

struct Handler;

// struct VoiceManager;
#[group]
#[commands(
    play, pause, resume, stop, skip, toloop, endloop, help, /*config*/ queue, playlist, lyrics
)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("the DiscordBot is ready");

        for guild in ready.guilds.iter() {
            match guild {
                GuildStatus::Offline(guild) => {
                    if format!("{}", guild.id.0) == env::var("GUILD_ID").unwrap() {
                        let channels = guild.id.channels(&ctx.http).await.unwrap();

                        channels
                            .get(&ChannelId(env::var("CHANNEL_ID").unwrap().parse().unwrap()))
                            .unwrap()
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.field("Bot reportandose 🚀", "Macri Bot", true).colour(
                                        Colour::from_rgb(
                                            rand::thread_rng().gen_range(0..255),
                                            rand::thread_rng().gen_range(0..255),
                                            rand::thread_rng().gen_range(0..255),
                                        ),
                                    )
                                })
                            })
                            .await
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}

pub async fn client_builder(token:&str) -> Client {
    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| {
            c.with_whitespace(false)
                .prefix(env::var("PREFIX").unwrap().as_str())
        });

    let client = Client::builder(&token)
        .framework(framemwork)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error when creating client");

    client
}

// functions which are called when a command is sent. For example: -play....
#[command]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let track_name: Vec<&str> = utils::message_to_vector(&msg.content[..]);

    music_bot::join(&ctx, &msg).await?;

    if track_name.len() == 2 {
        music_bot::play(&ctx, &msg, Some(track_name[1].to_string()), None).await?;
    }

    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::pause(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::resume(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::stop(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::skip(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn toloop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::to_loop(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn endloop(ctx: &Context, msg: &Message) -> CommandResult {
    music_bot::end_loop(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let iterator = vec![
        ("⏯️  -p", "reproducir canciones", false),
        ("🛑  -pause:", "pausar una cancion", false),
        ("🛑  -stop", "frenar definitivamente una cancion", false),
        ("⏯️  -resume", "reanudar una cancion pausada", false),
        ("⏭️  -skip", "saltear una cacion", false),
        ("♾️  -toloop", "repetir la cancion infinitamente", false),
        ("🔁  -endloop", "frenar la repeticion", false),
        //("💻  -config", "entrar en la configuracion del bot", false),
        ("⏯️  -playlist", "reproducir una playlist de spotify", false),
        (
            "📜  -lyrics",
            "obtener la letra de la cancion que se esta reproducioendo",
            false,
        ),
    ];

    utils::send_message_multi_line(iterator, ctx, msg).await;

    Ok(())
}

// #[command]
// async fn config(ctx: &Context, msg: &Message) -> CommandResult {Ok(())}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    queue::show_queue_list(ctx, msg).await?;

    Ok(())
}

#[command]
async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
    let play_list_name: Vec<&str> = utils::message_to_vector(&msg.content[..]);

    music_bot::join(&ctx, &msg).await?;

    if play_list_name.len() == 2 {
        music_bot::play(&ctx, &msg, None, Some(play_list_name[1])).await?;
    }
    Ok(())
}

#[command]
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