use rand::Rng;
use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{help, hook},
        Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, id::ChannelId, prelude::UserId},
    prelude::*,
    utils::Colour,
};
use songbird::SerenityInit;
use std::time::Duration;
use std::{collections::HashSet, env};

mod bot;
mod commands;
mod lyrics;
mod sources;
mod utils;

use commands::general::GENERAL_GROUP;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("the DiscordBot is ready");
        //
        //         for guild in ready.guilds.iter() {
        //             match guild {
        //                 GuildStatus::Offline(guild) => {
        //                     if format!("{}", guild.id.0) == env::var("GUILD_ID").unwrap() {
        //                         let channels = guild.id.channels(&ctx.http).await.unwrap();
        //
        //                         channels
        //                             .get(&ChannelId(env::var("CHANNEL_ID").unwrap().parse().unwrap()))
        //                             .unwrap()
        //                             .send_message(&ctx.http, |m| {
        //                                 m.embed(|e| {
        //                                     e.field("Bot reportandose 🚀", "Macri Bot", true).colour(
        //                                         Colour::from_rgb(
        //                                             rand::thread_rng().gen_range(0..255),
        //                                             rand::thread_rng().gen_range(0..255),
        //                                             rand::thread_rng().gen_range(0..255),
        //                                         ),
        //                                     )
        //                                 })
        //                             })
        //                             .await
        //                             .unwrap();
        //                     }
        //                 }
        //                 _ => {}
        //             }
        //         }
    }
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn on_unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    utils::send_message_single_line(
        "Error",
        format!("El comando {} no es correcto", unknown_command_name).as_str(),
        false,
        ctx,
        msg,
    )
    .await;
}

pub async fn client_builder(token: &str) -> Client {
    let framemwork = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .configure(|c| {
            c.with_whitespace(false)
                .prefix(env::var("PREFIX").unwrap().as_str())
        })
        .help(&HELP)
        .unrecognised_command(on_unknown_command);
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .framework(framemwork)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error when creating client");

    client
}
