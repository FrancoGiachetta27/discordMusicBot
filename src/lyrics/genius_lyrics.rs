use dotenv;
use genius_lyrics;
use genius_rs::Genius;
use rand::Rng;
use serenity::{
    framework::standard::CommandResult, model::channel::Message, prelude::*, utils::Colour,
};
use std::env;

pub async fn get_lyrics(ctx: &Context, msg: &Message, mut track_name: &str) -> CommandResult {
    dotenv::dotenv().expect(".env file not found");

    let genius = Genius::new(env::var("GENIUS_TOKEN").unwrap());

    for (i, word) in track_name.bytes().enumerate() {
        if word == b'(' {
            track_name = &track_name[..i - 1];
        }
    }

    let lyrics = match genius_lyrics::get_lyrics_from_url(
        &genius.search(&track_name).await.unwrap()[0].result.url,
    )
    .await
    {
        Ok(lyrics) => lyrics,
        Err(_err) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "❌ | No se han podido encontrar las lyrics de esa cancion",
                )
                .await
                .unwrap();
            return Ok(());
        }
    };

    if !lyrics.is_empty() {
        let mut count = 0;
        let mut track_lyrics: Vec<(&str, &str, bool)> = Vec::new();

        for (i, _word) in lyrics.bytes().enumerate() {
            if i == 1020
                || i == 2040
                || i == 3060
                || i == 4080
                || i == 5100
                || i == 6120
                || i == 7140
            {
                track_lyrics.push(("-", &lyrics[count..i], false));

                count = i;
            } else if i == lyrics.bytes().len() - 1 {
                track_lyrics.push(("-", &lyrics[count..i + 1], false));
            }
        }

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field(
                        format!("🎸 {} ", track_name),
                        "--------------------------------------------------------------",
                        false,
                    )
                    .fields(track_lyrics)
                    .colour(Colour::from_rgb(
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                    ))
                })
            })
            .await
            .unwrap();
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "❌ | No se han podido encontrar las lyrics de esa cancion",
            )
            .await
            .unwrap();
    }

    Ok(())
}
