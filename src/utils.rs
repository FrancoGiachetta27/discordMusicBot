use rand::Rng;
use serenity::{
    client::Context,
    model::channel::Message, 
    framework::standard::CommandResult, 
    utils::Colour
};
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn gethandler(ctx: &Context, msg: &Message) -> CommandResult<Option<Arc<Mutex<Call>>>> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guildId = guild.id;
    let manager = songbird::get(&ctx).await.unwrap().clone(); // gets the voice client

    let handlerLock = match manager.get(guildId) {
        Some(handler) => handler,
        None => {
            msg.reply(&ctx.http, "❌ | No estas en un canal de voz")
                .await?;

            return Ok(None);
        }
    };

    Ok(Some(handlerLock.to_owned()))
}

pub async fn sendMessageMultiLine(iterator:Vec<(&str,&str,bool)>, ctx: &Context, msg: &Message) {
    msg.channel_id
        .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.fields(iterator)
                    .colour(Colour::from_rgb(
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                    ))
                })
            } 
        )
        .await
        .expect("Coudln't send the message");
}

pub async fn sendMessageSingleLine(name:&str, value:&str, inline:bool, ctx: &Context, msg: &Message) {
    msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field(name,value,inline)
                    .colour(Colour::from_rgb(
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                        rand::thread_rng().gen_range(0..255),
                    ))
                })
            })
            .await
            .unwrap();
}

// get the song's or the playlist's name by conveting the message into a vector
pub fn MessageToVector(msg: &str) -> Vec<&str> {
    let bytes = msg.as_bytes();
    let mut stringVector = Vec::new();
    let cut = 0;

    for (i, &word) in bytes.iter().enumerate() {
        if word == b' ' {
            stringVector.push(&msg[cut..i]);
            stringVector.push(&msg[i..].trim());

            break;
        }
    }

    return stringVector;
}

// convert the message with the configuration into a vector
pub fn getConfig(msg: &str) -> Vec<&str> {
    let bytes = msg.as_bytes();
    let mut stringVector = Vec::new();
    let mut cut = 0;

    for (i, &word) in bytes.iter().enumerate() {
        if word == b' ' || i == bytes.len() - 1 {
            stringVector.push(msg[cut..i + 1].trim());

            cut = i;
        }
    }

    println!("{:?}", stringVector);

    return stringVector;
}
