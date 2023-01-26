use dotenv;
use genius_lyrics;
use genius_rs::Genius;
use serenity::{
    framework::standard::CommandResult, model::channel::Message, prelude::*, utils::Colour,
};
use std::env;

use crate::utils::get_rand_colors;

pub async fn get_lyrics(ctx: &Context, msg: &Message, mut track_name: &str) -> CommandResult {
    dotenv::dotenv().expect(".env file not found");

    let genius = Genius::new(env::var("GENIUS_TOKEN").unwrap());
    let (r_red, r_green, r_blue) = get_rand_colors();

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
        let mut _track_lyrics: Vec<(&str, &str, bool)> = Vec::new();

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field(
                        format!("🎸 {}", track_name),
                        "--------------------------------------------------------------",
                        false,
                    )
                    .colour(Colour::from_rgb(r_red, r_green, r_blue))
                })
            })
            .await
            .unwrap();

        println!("{lyrics}");

        for (i, _word) in lyrics.bytes().enumerate() {
            if i == 1020 || i == 2000 || i == 4000 || i == 6000 || i == 8000 || i == 10000 {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.content(format!("```css\n{}\n```", &lyrics[count..i]))
                    })
                    .await
                    .unwrap();

                count = i;
            } else if i == lyrics.bytes().len() - 1 {
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.content(format!("```css\n{}\n```", &lyrics[count..i]))
                    })
                    .await
                    .unwrap();
            }
        }
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
