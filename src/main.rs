use dotenv;
use rand::Rng;
use std::env;
use serenity:: {
    async_trait,
    utils::Colour,
    model::{channel::{Message}, gateway::{Ready}},
    framework::standard::{
        macros::{command, group},
        CommandResult,
        StandardFramework,
    },
    prelude::*,
};
use songbird::SerenityInit;

mod botFunctions;
mod stringToVector;
mod musicBot;
mod youtube;
mod queue;
mod spotify;

struct Handler;
// struct VoiceManager;
#[group]
#[commands(play,pause,resume,stop,skip,toloop,endloop,help,config,queue,playlist)]
struct General;

#[async_trait]
// functions related to event_handler
impl EventHandler for Handler {
    async fn message(&self, ctx:Context, msg: Message) {

    }

    async fn ready(&self, ctx:Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env file not found");

    let token:String = env::var("TOKEN").unwrap();

    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| c.with_whitespace(false).prefix(env::var("PREFIX").unwrap().as_str()));

    let mut client = Client::builder(&token).framework(framemwork).event_handler(Handler).register_songbird().await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}

// functions which are called when a command is sent. For example: -play....
#[command]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let trackName:Vec<&str> = stringToVector::getName(&msg.content[..]);

    botFunctions::join(&ctx, &msg).await?;

    if trackName.len() == 2 {
        musicBot::play(&ctx,&msg,Some(trackName[1]),None).await?;
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
    musicBot::stop(&ctx,&msg).await?;
    botFunctions::leave(&ctx,&msg).await?;

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
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.field("👨‍💻 Comandos:",".",true)
            .fields(vec![
                ("⏯️  -p", "reproducir canciones", false),
                ("🛑  -pause:", "pausa una cacion", false),
                ("🛑  -stop", "frena definitivamente una cancion", false),
                ("⏯️  -resume", "reanuda una cancion pausada", false),
                ("⏭️  -skip", "saltea una cacion", false),
                ("🔁  -toloop", "repetir la cancion infinitamente", false),
                ("🔁  -endloop", "frena la repeticion", false),
                ("💻  -config", "entrar en la configuracion del bot", false),
                ("⏯️  -playlist", "reproducir una playlist de spotify", false),
            ])
            .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
        })
    }).await.unwrap();

    Ok(())
}

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let config:Vec<&str> = stringToVector::getConfig(&msg.content[..]);

    if config.len() == 1 {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field("👨‍💻 Configuracion:",".",true)
                .fields(vec![
                    ("-config prefix", "valor", false),
                ])
                .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
            })
        }).await.unwrap();
    }else if config.len() == 3 {
        match config[1] {
            "prefix" => env::set_var("PREFIX", config[2]),
            _ => {
                msg.channel_id.say(&ctx.http,"❌ | Ese comando no es correcto").await?;
            }
        }
    }else {
        msg.channel_id.say(&ctx.http,"❌ | Ese comando no es correcto").await?;
    }

    msg.channel_id.say(&ctx.http,format!("{}",env::var("PREFIX").unwrap())).await?;

    
    Ok(())
}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    queue::showQueueList(ctx, msg).await?;

    Ok(())
}

#[command]
async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
    let playListName:Vec<&str> = stringToVector::getName(&msg.content[..]);

    botFunctions::join(&ctx, &msg).await?;

    if playListName.len() == 2 {
        musicBot::play(&ctx,&msg,None,Some(playListName[1])).await?;
    }
    Ok(())
}