use dotenv;
use rand::Rng;
use std::env;
use serenity:: {
    utils::Colour,
    model::{channel::{Message}, gateway::{Ready}},
    framework::standard::{
        CommandResult,
    },
    prelude::*,
};
use genius_rs::Genius;
use genius_lyrics;

pub async fn getLyrics(ctx: &Context, msg: &Message, mut trackName: &str) -> CommandResult {
    dotenv::dotenv().expect(".env file not found");

    let genius = Genius::new(env::var("GENIUS_TOKEN").unwrap());

    for (i, word) in trackName.bytes().enumerate() { 
        if word == b'(' {
            trackName = &trackName[..i - 1];
        }
    }

    let response = genius.search(&trackName).await.unwrap();
    let lyrics = match genius_lyrics::get_lyrics_from_url(&response[0].result.url).await {
        Ok(lyrics) => lyrics,
        Err(why) => {
            msg.channel_id.say(&ctx.http,"❌ | No se han podido encontrar las lyrics de esa cancion").await.unwrap();
            return Ok(());
        }
    };  

    if !lyrics.is_empty() {
        let mut count = 0;
        let mut tackLyrics:Vec<(&str,&str,bool)> = Vec::new();

        for (i,word) in lyrics.bytes().enumerate() {
            if i == 1020 || i == 2040 || i == 3060 || i == 4080 ||  i == 5100 ||  i == 6120 ||  i == 7140{
                tackLyrics.push(("-", &lyrics[count..i],false));

                count = i;
            }else if i == lyrics.bytes().len() - 1 {
                tackLyrics.push(("-", &lyrics[count..i + 1],false));
            }
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field(format!("🎶 {} ", trackName), "--------------------------------------------------------------", false)
                .fields(
                    tackLyrics
                )
                .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
            })
        }).await.unwrap();
    }else {
        msg.channel_id.say(&ctx.http,"❌ | No se han podido encontrar las lyrics de esa cancion").await.unwrap();
    }

    Ok(())
}
