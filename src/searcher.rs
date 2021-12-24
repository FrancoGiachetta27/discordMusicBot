use serenity::{
    model::{channel::Message},
    client::Context,
    framework::standard::{
        CommandResult,
    }
};
use songbird::{
    input::{
        Input,
        ytdl,
        ytdl_search
    },
};

pub async fn getSource(ctx: &Context, msg: &Message, trackName:&str) -> CommandResult<Option<Input>> {
    let source:Input;

    if  trackName.starts_with("https") || trackName.starts_with("http") {
        source = match ytdl(&trackName).await { //gets the track from youtube by the url
            Ok(input) => input,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id.say(&ctx.http, "No se ha podido encontrar esa cacion").await?;

                return Ok(None);
            }
        };
    }else {
        source = match ytdl_search(&trackName).await { //gets the track from youtube by the song's name
            Ok(input) => input,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id.say(&ctx.http, "No se ha podido encontrar esa cacion").await?;

                return Ok(None);
            }
        };
    }

    Ok(Some(source))
} 