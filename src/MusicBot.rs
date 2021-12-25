use serenity::{
    model::{guild::Guild,channel::Message},
    prelude::*,
    client::Context,
    framework::standard::{
        CommandResult,
    }
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
    
    let trackQueue = match queue::queue(ctx,msg,Some(trackName),&mut handler).await? {
        Some(queue) => queue,
        None => {
            return Ok(());
        }
    };
    
    while !trackQueue.is_empty() {
        let track = match trackQueue.dequeue(0) {
            Some(track) => track,
            None => {
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

        if let songbird::tracks::PlayMode::Pause = trackStatus.playing {
            track.play()?;
            trackQueue.modify_queue(|queue| queue.remove(0)); 
        }else if let songbird::tracks::PlayMode::Stop = trackStatus.playing {
            track.play()?;
            trackQueue.modify_queue(|queue| queue.remove(0)); //takes the skiped track, which is now the first item in the trackQueue and deletes it
        };
    }

    msg.channel_id.say(&ctx.http,"No hay canciones para reporducir...").await?;

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

    trackQueue.skip();

    trackQueue.modify_queue(|queue| { //takes the skiped track, which is now the first item in the trackQueue and deletes it
        queue.remove(0);
    });

    println!("Queue {:?}", trackQueue);

    Ok(())
}