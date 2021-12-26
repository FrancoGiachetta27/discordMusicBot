use serenity::{
    model::{channel::Message},
    prelude::*,
    client::Context,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    tracks::{Queued, PlayMode, TrackState, TrackQueue, TrackHandle},
};

use crate::stringToVector;
use crate::queue;

//play a track
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client
    let trackName:&str = stringToVector::convert(&msg.content[..])[1];
    
    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;
    
    let trackQueue: &TrackQueue = match queue::queue(ctx,msg,Some(trackName),&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };

    let mut currentTrack = match trackQueue.current() {
        Some(track) => Some(track),
        None => {
            let nextTrack = match trackQueue.dequeue(0) {
                Some(track) => Some(track.handle()),
                None => None
            };

            nextTrack
        }
    };

    while !trackQueue.is_empty() {
        currentTrack = match trackQueue.current() {
            Some(track) => Some(track),
            None => {
                let nextTrack = match trackQueue.dequeue(0) {
                    Some(track) => Some(track.handle()),
                    None => None
                };
    
                nextTrack
            }
        };    

        let trackStatus = match &currentTrack {
            Some(currentTrack) => match currentTrack.get_info().await  {  
                Ok(status) => status,
                Err(why) => {
                    println!("Error {}", why);
                        
                    return Ok(());
                }
            },
            None => return Ok(())
        };

        if let PlayMode::Stop = trackStatus.playing {
            if let Some(currentTrack) = &currentTrack {
                currentTrack.play()?;
            };

            trackQueue.modify_queue(|queue| queue.remove(0)); 
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
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz").await?;

            return Ok(());
        },
    };

    let mut handler = handlerLock.lock().await;

    {
        let trackQueue = match queue::queue(ctx,msg,None,&mut handler).await? {
            Some(queue) => queue,
            None => {
                return Ok(());
            }
        };

        trackQueue.modify_queue(|queue| queue.clear()); // deletes the trackQueue
    }
    
    handler.stop();

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

    match trackQueue.current() {
        Some(track) => {
            track.pause()?;
        },
        None => {
            msg.reply(&ctx.http, "❌ | No hay mas canciones para reproducir").await?;
        }
    }

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

    match trackQueue.current() {
        Some(track) => track.stop()?,
        None => ()
    };

    trackQueue.skip()?;

    trackQueue.modify_queue(|queue| { //takes the skiped track, which is now the first item in the trackQueue and deletes it
        queue.remove(0);
    });

    println!("Queue {:?}", trackQueue);

    Ok(())
}
