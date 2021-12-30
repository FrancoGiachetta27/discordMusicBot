use rand::Rng;
use serenity::{
    model::{channel::Message},
    client::Context,
    utils::Colour,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    Call,
    tracks::{ TrackQueue },
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
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field("🎙️ Se ha añadido una cancion a la lista de canciones:", name, true)
                     .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
                });
    
                m
            }).await.expect("Coudln't send the message");
        }

        handler.enqueue_source(source);
    }
    
    Ok(Some(handler.queue()))   
} 