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
    tracks::{TrackHandle, TrackQueue},
    SerenityInit,
};
use std::env;

mod bot;
mod geniusLyrics;
mod sources;
mod utils;

use bot::{musicBot, queue};
use geniusLyrics::geniusLyrics::getLyrics;

struct Handler;

// struct VoiceManager;
#[group]
#[commands(
    play, pause, resume, stop, skip, toloop, endloop, help, /*config*/ queue, playlist, lyrics
)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {}

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

pub async fn clientBuilder(token:&str) -> Client {
    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| {
            c.with_whitespace(false)
                .prefix(env::var("PREFIX").unwrap().as_str())
        });

    let mut client = Client::builder(&token)
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
    let trackName: Vec<&str> = utils::MessageToVector(&msg.content[..]);

    musicBot::join(&ctx, &msg).await?;

    if trackName.len() == 2 {
        musicBot::play(&ctx, &msg, Some(trackName[1]), None).await?;
    }

    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::pause(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::resume(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::stop(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::skip(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn toloop(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::toLoop(&ctx, &msg).await?;

    Ok(())
}

#[command]
async fn endloop(ctx: &Context, msg: &Message) -> CommandResult {
    musicBot::endLoop(&ctx, &msg).await?;

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

    utils::sendMessageMultiLine(iterator, ctx, msg).await;

    Ok(())
}

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {Ok(())}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    queue::showQueueList(ctx, msg).await?;

    Ok(())
}

#[command]
async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
    let playListName: Vec<&str> = utils::MessageToVector(&msg.content[..]);

    musicBot::join(&ctx, &msg).await?;

    if playListName.len() == 2 {
        musicBot::play(&ctx, &msg, None, Some(playListName[1])).await?;
    }
    Ok(())
}

#[command]
async fn lyrics(ctx: &Context, msg: &Message) -> CommandResult {
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

    let trackQueue = handler.queue();

    if let Some(track) = trackQueue.current() {
        getLyrics(
            &ctx,
            &msg,
            &mut track.metadata().title.as_ref().unwrap().as_str(),
        )
        .await?;
    }
    Ok(())
}