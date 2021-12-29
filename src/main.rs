use dotenv;
use std::env;
use serenity:: {
    async_trait,
    model::{channel::{Message}, gateway::{Ready}},
    framework::standard::{
        macros::{check, command, group, help, hook},
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

struct Handler;
// struct VoiceManager; 
#[group]
#[commands(play,pause,resume,stop,skip,toloop,endloop)]
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

    let mut client = Client::builder(token).framework(framemwork).event_handler(Handler).register_songbird().await.expect("Error when creating client");
    
    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}

// functions which are called when a command is sent. For example: -play.....

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let trackName:Vec<&str> = stringToVector::convert(&msg.content[..]);

    botFunctions::join(&ctx, &msg).await?;

    if trackName.len() == 2 {
        musicBot::play(&ctx,&msg,Some(trackName[1])).await?;
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

