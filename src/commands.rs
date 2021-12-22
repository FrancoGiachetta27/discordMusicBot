use serenity::{
    model::{channel::Message, gateway::{Activity}},
    prelude::*,
    client::Context,
};

use songbird::SerenityInit;

mod Commands{
    pub async fn play() {}

    pub async fn pause() {}

    pub async fn stop() {}

    pub async fn skip() {}

    pub async fn resume() {}

}
