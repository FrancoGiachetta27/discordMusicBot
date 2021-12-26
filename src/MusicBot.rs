use serenity::{
    model::{channel::Message},
    prelude::*,
    client::Context,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    tracks::{PlayMode, TrackState, TrackQueue, TrackHandle},
};

use crate::stringToVector;
use crate::queue;

//play a track
pub async fn play(ctx: &Context, msg: &Message, trackName:Option<&str>) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
    let playable = true;
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;
    
    let trackQueue: &TrackQueue = match queue::queue(ctx,msg,trackName,&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    let mut currentTrack = trackQueue.current();
    
    let mut trackStatus = if let Some (currentTrack) = &currentTrack {
        Some(currentTrack.get_info().await?)
    }else{
        return Ok(());
    };
    
   
    if let Some(currentTrack) = &currentTrack {
        if let Some(trackStatus) = &trackStatus {
            if let PlayMode::Play = trackStatus.playing {
                println!("current {:?} \n", trackQueue.current());
            }else if let PlayMode::Pause = trackStatus.playing {
                trackQueue.modify_queue(|queue| queue.remove(0));
            };
        }
    } else if let None = &currentTrack {
        currentTrack =  trackQueue.current();

        trackStatus = if let Some (currentTrack) = &currentTrack {
            Some(currentTrack.get_info().await?)
        }else{
            return Ok(());
        };
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
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = match queue::queue(ctx,msg,None,&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.stop(); // stops the track and deletes the trackQueue

    Ok(())
}

//pauses the current track
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone();
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = match queue::queue(ctx,msg,None,&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.pause()?;

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
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = match queue::queue(ctx,msg,None,&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    match trackQueue.current() {
        Some(track) => {
            track.play()?;
        },
        None => {
            msg.reply(&ctx.http, "❌ | No hay mas canciones para reproducir").await?;
        }
    } 

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
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    let trackQueue = match queue::queue(ctx,msg,None,&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    trackQueue.skip()?;

    trackQueue.modify_queue(|queue| {
        println!("firstTrack {:?}",queue);

        queue.remove(0); // remove the skipped track
    });

    if let Some(newTrack) = trackQueue.current() {
        newTrack.play()?;
    }

    Ok(())
}
