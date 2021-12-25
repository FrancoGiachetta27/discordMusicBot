use serenity::{
    model::{guild::Guild,channel::Message},
    prelude::*,
    client::Context,
    framework::standard::{
        CommandResult,
    }
};


// makes the bot join the channel where the message's author is, if not in any channel it won't work 
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap(); // gets an instance of the server where the bot is in
    let guildId = guild.id;
    let channelId = guild.voice_states.get(&msg.author.id).and_then(|voiceState| voiceState.channel_id); //gets the channel id where the message's author is

    let connectTo = match channelId {
        Some(channel) => channel,
        None => {
            msg.channel_id.say(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(&ctx).await.unwrap().clone(); //creates a voice client

    let handler = manager.join(guildId, connectTo).await;

    Ok(())
}

// makes the bot leave a channel
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    let hasHandler = manager.get(guildId).is_some();

    if hasHandler {
        if let Err(why) = manager.remove(guildId).await {
            msg.channel_id.say(&ctx.http, "❌ | Error al desconectar el bot").await?;
        }

        msg.channel_id.say(&ctx.http, "Bot desconectado").await?;
    }

    Ok(())
}
