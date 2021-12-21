use serenity:: {
    async_trait,
    model::{voice::VoiceState,id::GuildId,channel::{Message}, gateway::{Ready}},
    prelude::*,
};

mod stringToVec;
mod commandFunctions;

struct Handler;

const TOKEN:&str = "OTE5Njk0MTczMDU2MTcyMDQy.YbZh8Q.RPgY_z3rRDHZqtPkQU47kQhN0vM";

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(&self, ctx:Context, arg2: Option<GuildId>, new: VoiceState) {
        let membersInChannel = match new.member {
            Some(members) => members,
            None => return 
        };

        println!("Channel: {:?}\n member: {}",new.channel_id, membersInChannel.user.name);
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
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(TOKEN).event_handler(Handler).await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}
