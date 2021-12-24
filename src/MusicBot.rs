use serenity::{
    model::{guild::Guild,channel::Message},
    prelude::*,
    client::Context,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    Call,
    SerenityInit, 
    tracks::{TrackQueue},
    input::{
        Input,
        ytdl,
        ytdl_search
    },
};
use tokio::sync::MutexGuard;

use crate::stringToVector;
use crate::queue;

//play a track
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
    let trackName:&str = stringToVector::convert(&msg.content[..])[1];
    let source:Input;
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "Not in a voice channel").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;
    
    let trackQueue = match queue::queue(ctx,msg,Some(trackName),&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(())
        }
    };
    
    while trackQueue.is_empty() {
        let track = match trackQueue.dequeue(0) {
            Some(track) => track,
            None => {
                msg.channel_id.say(&ctx.http,"No hay mas canciones para reporducir...").await?;

                return Ok(());
            }
        };

        let trackStatus = match track.get_info().await {
            Ok(status) => status,
            Err(why) => {
                println!("Error {}", why);
                
                return Ok(());
            }
        };

        while let songbird::tracks::PlayMode::Play = trackStatus.playing {

       }
    }

    Ok(())
}

//stop the track permanently
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "Not in a voice channel").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;
    
    handler.stop();

    Ok(())
}

//Unpauses a the the bot
pub async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "Not in a voice channel").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    Ok(())
}

//skips the current track
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "Not in a voice channel").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    Ok(())
}