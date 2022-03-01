use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::input::{ytdl, ytdl_search, Input};

// gets the source of the track from youtube and returns it
pub async fn getSource<'a>(
    ctx: &Context,
    msg: &Message,
    trackName: &str,
) -> CommandResult<Option<Input>> {
    let source: Input;

    if trackName.starts_with("https") || trackName.starts_with("http") {
        source = match ytdl(&trackName).await {
            //gets the track from youtube by the url
            Ok(input) => input,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, "❌ | No se ha podido encontrar esa cacion")
                    .await?;

                return Ok(None);
            }
        };
    } else {
        source = match ytdl_search(&trackName).await {
            //gets the track from youtube by the song's name
            Ok(input) => input,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, format!("❌ | Ha ocurrido un error al buscar '{}', puede ser por no ser encontrada o por restricción de edad", trackName))
                    .await?;

                return Ok(None);
            }
        };
    }

    Ok(Some(source))
}
