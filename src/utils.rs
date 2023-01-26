use rand::Rng;
use serenity::{client::Context, model::channel::Message, utils::Colour};

pub async fn send_message_multi_line(
    iterator: Vec<(&str, &str, bool)>,
    ctx: &Context,
    msg: &Message,
) {
    let (r_red, r_green, r_blue) = get_rand_colors();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.fields(iterator)
                    .colour(Colour::from_rgb(r_red, r_green, r_blue))
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
    let (r_red, r_green, r_blue) = get_rand_colors();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field(name, format!("```css\n{}\n```", value), inline)
                    .colour(Colour::from_rgb(r_red, r_green, r_blue))
            })
        })
        .await
        .unwrap();
}

pub fn get_rand_colors() -> (u8, u8, u8) {
    (
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
    )
}
