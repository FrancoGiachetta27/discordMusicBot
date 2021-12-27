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
    tracks::{ 
        TrackQueue,
        TrackState,
        TrackHandle
    },
};

use crate::youtube;

//enqueues the source of the track found on youtube and returns the full  
pub async fn queue<'a>(ctx:&Context, msg:&Message, trackName:Option<&str>, handler:&'a mut Call) -> CommandResult<Option<&'a TrackQueue>> {
    if let Some(trackName) = trackName {
        let source = match youtube::getSource(&ctx,&msg,&trackName).await? {
            Some(source) => source,
            None => { return Ok(None); }
        };

        if let Some(name) = &source.metadata.title {
            msg.channel_id.say(&ctx.http,format!("Se ha agregado {} a la lista de canciones",name)).await?;
        }

        handler.enqueue_source(source);
    }

    println!("Queue {:?} \n", handler.queue().current_queue());
    
    Ok(Some(handler.queue()))   
} 