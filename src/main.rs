use serenity:: {
    async_trait,
    futures::channel::mpsc::UnboundedSender,
    gateway::InterMessage,
    model::{voice::VoiceState,id::{GuildId,UserId},channel::{Message,ChannelType}, gateway::{Ready}},
    client::bridge::voice::VoiceGatewayManager,
    cache::Cache,
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    prelude::*,
};
use songbird::SerenityInit;

mod stringToVec;
mod commands;

struct Handler;
// struct VoiceManager;

const TOKEN:&str = "OTE5Njk0MTczMDU2MTcyMDQy.YbZh8Q.RPgY_z3rRDHZqtPkQU47kQhN0vM";

#[async_trait] 
// functions related to event_handler
impl EventHandler for Handler {
    async fn message(&self, ctx:Context, msg: Message) {
        let msgVec:Vec<&str> = stringToVec::convert(&msg.content); 

        let guild = match msg.guild_id {
            Some(id) => id,
            None => return 
        };

        if msgVec[0] == "-p" {
            let guildChannels = guild.channels(&ctx.http).await.unwrap();

            for (id,chl) in guildChannels.iter() {
                let found = false;

                match chl.kind {
                    ChannelType::Voice => chl,
                    _ => continue
                };

                let members = chl.members(&ctx.cache).await.unwrap();

                if !members.is_empty(){
                    for mem in members.iter() {
                        if mem.user.id == msg.author.id {
                            println!("found");

                            if let Err(why) = guild.move_member(&ctx.http,919694173056172042,id).await {
                                println!("Error: {}", why);
                            };

                            break; 
                        }
                    }
                }

                if found {
                    break;
                }
            }

        }else if msgVec[0] == "-stop" {
            
        }else if msgVec[0] == "-skip" {
            
        }else if msgVec[0] == "-resume" {
            
        }else if msgVec[0] == "-pause" {
            
        }
    }

    async fn ready(&self, ctx:Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// #[async_trait] 
// impl VoiceGatewayManager for VoiceManager {
//     async fn initialise(&self, shard_count: u64, user_id: UserId) {
//         println!("nice");
//     }

//     async fn register_shard<'life0>(&'life0 self, shard_id: u64, sender: UnboundedSender<InterMessage>) {

//     }
    
//     async fn deregister_shard(&self, shard_id: u64) {

//     }

//     async fn server_update<'life0, 'life1, 'life2>(&'life0 self, guild_id: GuildId, endpoint: &'life1 Option<String>, token: &'life2 str) {
        
//     }

//     async fn state_update<'life0, 'life1>(&'life0 self, guild_id: GuildId, voice_state: &'life1 VoiceState) {}
    
// }

#[tokio::main]
async fn main() {
    let mut client = Client::builder(TOKEN).event_handler(Handler).register_songbird().await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}

