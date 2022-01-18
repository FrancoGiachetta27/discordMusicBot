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

    let token:String =  match env::var("TOKEN") {
        Ok(val) => val,
        Err(why) => panic!("{}", why)
    };

    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| c.with_whitespace(false).prefix("-"));

    let mut client = Client::builder(&token).framework(framemwork).event_handler(Handler).register_songbird().await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}

// functions which are called when a command is sent. For example: -play....
#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let trackName:Vec<&str> = stringToVector::convert(&msg.content[..]);

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
            e.fields(vec![
                ("Comandos:", "", true),
                ("-play", "reproducir canciones", true),
                ("-pause:", "pausa una cacion", true),
                ("-stop", "frena definitivamente una cancion", true),
                ("-resume", "reanuda una cancion pausada", true),
                ("-skip", "saltea una cacion", true),
                ("-toloop", "repetir la cancion infinitamente", true),
                ("-endloop", "frena la repeticion", true),
                ("-config","entrar en la configuracion del bot",true)
            ])
            .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
        });

        m
    }).await.expect("Coudln't send the message");

    println!("{}", msg.channel_id);

    Ok(())
}

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {

    Ok(())
}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    queue::showQueueList(ctx, msg).await?;

    Ok(())
}

#[command]
async fn playlist(ctx: &Context, msg: &Message) -> CommandResult {
    let playListName:Vec<&str> = stringToVector::convert(&msg.content[..]);

    botFunctions::join(&ctx, &msg).await?;

    if playListName.len() == 2 {
        musicBot::play(&ctx,&msg,None,Some(playListName[1])).await?;
    }
    Ok(())
}
