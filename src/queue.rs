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

use crate::searcher;

//enqueues the source of the track found on youtube and returns the full  
pub async fn queue<'a>(ctx:&Context, msg:&Message, trackName:Option<&str>, handler:&'a mut Call) -> CommandResult<Option<&'a TrackQueue>> {
    if let Some(trackName) = trackName {
        let source = match searcher::getSource(&ctx,&msg,&trackName).await? {
            Some(source) => source,
            None => { return Ok(None); }
        };

        if let Some(name) = &source.metadata.title {
            msg.channel_id.say(&ctx.http,format!("Se ha agregado {} a la lista de canciones",name)).await?;
        }

        handler.enqueue_source(source);
    }

    println!("Queue {:?}", handler.queue().current_queue());
    
    Ok(Some(handler.queue()))   
} 

pub async fn dequeue(ctx: &Context, msg: &Message, trackQueue:&TrackQueue) -> CommandResult<(Option<TrackState>,Option<TrackHandle>)>{
    let currentTrack = match trackQueue.current() {
        Some(track) => Some(track),
        None => {
            let nextTrack = match trackQueue.dequeue(0) {
                Some(track) => {
                    trackQueue.modify_queue(|queue| queue.remove(0)); 
                    
                    track.handle() 
                },
                None => { 
                    msg.channel_id.say(&ctx.http,"No hay mas caciones para reproducir").await?;
                    return Ok((None,None)); 
                } 
            };

            Some(nextTrack)
        }
    };    

    let trackStatus = if let Some (currentTrack) = &currentTrack {
        currentTrack.get_info().await?
    }else{
        return Ok((None,None));
    };

    Ok((Some(trackStatus),currentTrack))
}