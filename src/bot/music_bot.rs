use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::tracks::TrackQueue;
use std::time::Duration;

use crate::bot::queue;
use crate::utils;

// makes the bot join the channel where the message's author is, if not in any channel it won't work
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "No estas en un canal de voz").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

// makes the bot leave a channel
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = &msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild_id).await {
            msg.channel_id
                .say(&ctx.http, "❌ | Error al desconectar el bot")
                .await?;
        }

        msg.channel_id.say(&ctx.http, "Bot desconectado").await?;
    }
    Ok(())
}
