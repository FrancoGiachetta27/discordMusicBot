use rand::Rng;
use serenity::{client::Context, model::channel::Message, utils::Colour};

pub async fn send_message_multi_line(
    iterator: Vec<(&str, &str, bool)>,
    ctx: &Context,
    msg: &Message,
) {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.fields(iterator).colour(Colour::from_rgb(
                    rand::thread_rng().gen_range(0..255),
                    rand::thread_rng().gen_range(0..255),
                    rand::thread_rng().gen_range(0..255),
                ))
            })
        })
        .await
        .expect("Coudln't send the message");
}

pub async fn send_message_single_line(
    name: &str,
    value: &str,
    inline: bool,
    ctx: &Context,
    msg: &Message,
) {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field(name, value, inline).colour(Colour::from_rgb(
                    rand::thread_rng().gen_range(0..255),
                    rand::thread_rng().gen_range(0..255),
                    rand::thread_rng().gen_range(0..255),
                ))
            })
        })
        .await
        .unwrap();
}
