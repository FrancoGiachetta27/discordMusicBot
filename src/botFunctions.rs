use serenity::{
    model::{guild::Guild,channel::Message, gateway::{Activity}},
    prelude::*,
    client::Context,
    framework::standard:: {
        CommandResult,
    }
};
use songbird::{
    SerenityInit,
    driver::Driver, 
    ytdl, 
    tracks::create_player
};

pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let channelId = guild.voice_states.get(&msg.author.id).and_then(|voiceState| voiceState.channel_id);

    let connectTo = match channelId {
        Some(channel) => channel,
        None => {
            msg.channel_id.say(&ctx.http, "No estas en un canal de voz").await;

            return Ok(());
        }
    };

    let manager = songbird::get(&ctx).await.unwrap().clone();

    let handler = manager.join(guildId, connectTo).await;

    Ok(())
}

pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    let hasHandler = manager.get(guildId).is_some();

    if hasHandler {
        if let Err(why) = manager.remove(guildId).await {
            msg.channel_id.say(&ctx.http, "Error al desconectar el bot");
        }

        msg.channel_id.say(&ctx.http, "Bot desconectadoÑ");
    }
    Ok(())
}

pub async fn play(ctx: &Context, msg: &Message) -> CommandResult{
    let mut handler:Driver = Default::default();
    let source = ytdl("https://www.youtube.com/watch?v=J0N1yY937qg").await?;
    let (audio,auido_handler) = create_player(source);

    handler.play(audio);
    
    Ok(())
}

pub async fn stop() {
    
}

pub async fn resume() {
    
}

pub async fn skip() {

}

pub async fn queue() {

}


