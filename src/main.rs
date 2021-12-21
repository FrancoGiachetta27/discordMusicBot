use std::result::Result;
use serenity:: {
    async_trait,
    model::{event::ThreadMembersUpdateEvent,channel::{Message}, gateway::{Ready}},
    prelude::*,
};

mod stringToVec;
mod commandFunctions;

struct Handler;

const TOKEN:&str = "OTE5Njk0MTczMDU2MTcyMDQy.YbZh8Q.BHeKuu4njeRBkqK4mr_tsLv3mFY";

#[async_trait]
impl EventHandler for Handler {
    async fn thread_members_update(&self,ctx:Context,thread_members_update: ThreadMembersUpdateEvent) {
        println!("te has unido al canal con id {}", thread_members_update.id);
    }

    async fn message(&self, ctx:Context, msg: Message) {
        let msgVec:Vec<&str> = stringToVec::convert(&msg.content); 
        
        let channel = match ctx.http.get_channels(893227377704972338).await {
            Ok(channel) => channel,
            Err(why) => return ,
        };

            if msgVec[0] == "-p" {
                
            }else if msgVec[0] == "-stop" {

                if let Err(why) = msg.channel_id.say(&ctx.http, "cancion skipeada").await {
                    println!("Error sending message: {:?}", why);
                }
            }else if msgVec[0] == "-skip" {

                if let Err(why) = msg.channel_id.say(&ctx.http, "reproduccion frenada").await {
                    println!("Error sending message: {:?}", why);
                }
            }else {

                if let Err(why) = msg.channel_id.say(&ctx.http, msg.channel_id).await {
                    println!("Error sending message: {:?}", why);
                }
            }
    }

    async fn ready(&self, ctx:Context, ready:Ready) {
        let channel = match ctx.http.get_channels(893227377704972338).await {
            Ok(channel) => channel,
            Err(why) => return ,
        };
        for chl in channel.iter() {
            println!("Channel name {} \n Channel type {:?}", chl.name, chl.kind);
        }

        if let Err(why) = ctx.http.get_channel_active_threads(893227377704972341).await {
            println!("error {}", why);
        };
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(TOKEN).event_handler(Handler).await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}
