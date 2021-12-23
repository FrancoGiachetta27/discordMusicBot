use serenity:: {
    async_trait,
    futures::channel::mpsc::UnboundedSender,
    gateway::InterMessage,
    model::{voice::VoiceState,id::{GuildId,UserId},channel::{Message,ChannelType}, gateway::{Ready}},
    client::bridge::voice::VoiceGatewayManager,
    cache::Cache,
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args,
        CommandGroup,
        CommandOptions,
        CommandResult,
        StandardFramework,
    },
    prelude::*,
};
use songbird::SerenityInit;

mod botFunctions;

struct Handler;
// struct VoiceManager; 
#[group]
#[commands(play,pause,resume,skip)]
struct General;

const TOKEN:&str = "OTE5Njk0MTczMDU2MTcyMDQy.YbZh8Q.RPgY_z3rRDHZqtPkQU47kQhN0vM";

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
    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| c.with_whitespace(true).prefix("-"));

    let mut client = Client::builder(TOKEN).framework(framemwork).event_handler(Handler).register_songbird().await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    
    botFunctions::join(&ctx, &msg).await?;
    botFunctions::play(ctx,msg).await?;

    if let Err(why) = msg.channel_id.say(&ctx.http,"ready to play").await {
        println!("Error: {}",why);
    };
    
    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http,"ready to pause").await {
        println!("Error: {}",why);
    };
    
    Ok(())
}

#[command]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http,"ready to resume").await {
        println!("Error: {}",why);
    };

    Ok(())
}

#[command]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    botFunctions::leave(&ctx,&msg).await?;

    if let Err(why) = msg.channel_id.say(&ctx.http,"readu to skip").await {
        println!("Error: {}",why);
    };

    Ok(())
}


