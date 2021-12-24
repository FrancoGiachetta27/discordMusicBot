use serenity::{
    model::{channel::Message},
    client::Context,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    Call,
    tracks::{TrackQueue},
};

use crate::searcher;

//enqueues the source of the track found on youtube and returns the full  
pub async fn queue<'a>(ctx:&Context, msg:&Message, trackName:Option<&str>, handler:&'a mut Call) -> CommandResult<Option<&'a TrackQueue>> {
    if let Some(trackName) = trackName {
        let source = match searcher::getSource(&ctx,&msg,&trackName).await? {
            Some(source) => source,
            None => {
                msg.reply(&ctx.http, "No se encontro esa cancion").await?;
            
                return Ok(None);
            }
        };

        handler.enqueue_source(source);
    }

    Ok(Some(handler.queue()))   
} 