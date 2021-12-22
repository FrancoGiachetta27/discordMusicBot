use serenity:: {
    async_trait,
    model::{voice::VoiceState,id::GuildId,channel::{Message, Channel, ChannelType}, gateway::{Ready}},
    cache::Cache,
    prelude::*,
};

mod stringToVec;
mod commands;

struct Handler;

const TOKEN:&str = "OTE5Njk0MTczMDU2MTcyMDQy.YbZh8Q.RPgY_z3rRDHZqtPkQU47kQhN0vM";

#[async_trait]
impl EventHandler for Handler {
   // async fn voice_state_update(&self, ctx:Context, arg2: Option<GuildId>, new: VoiceState) {
        // let membersInChannel = match new.member {
        //     Some(member) => member,
        //     None => return 
        // };

        // println!("Channel: {:?}\n member: {}",new.channel_id, membersInChannel.user.name);
   // }

    async fn message(&self, ctx:Context, msg: Message) {
        let msgVec:Vec<&str> = stringToVec::convert(&msg.content); 

        let guildId = match msg.guild_id {
            Some(id) => id,
            None => return 
        };

        if msgVec[0] == "-p" {
            let guildChannels = guildId.channels(&ctx.http).await.unwrap();
            let mut i = 0;

            for (id,chl) in guildChannels.iter() {
                
                match chl.kind {
                    ChannelType::Voice => chl,
                    _ => continue
                };

                let members = chl.members(&ctx.cache).await.unwrap();

                while true {
                    if !members.is_empty(){
                        if members[i].user.id == msg.author.id {
                            println!("found {} in channel: {}", msg.author.id, chl.name);

                            break;
                        }
                        
                        i += 1;
                    }
                }

                println!("id: {},channel kind: {:?}",id, chl.name);
            }

        }else if msgVec[0] == "-stop" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "cancion skipeada").await {
                println!("Error sending message: {:?}", why);
            }
        }else if msgVec[0] == "-skip" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "reproduccion frenada").await {
                println!("Error sending message: {:?}", why);
            }
        }else if msgVec[0] == "-resume" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hola").await {
                println!("Error sending message: {:?}", why);
            }
        }else if msgVec[0] == "-pause" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "hola").await {
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
