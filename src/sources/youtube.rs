use serenity::{client::Context, framework::standard::CommandResult, model::channel::Message};
use songbird::input::restartable::Restartable;

// gets the source of the track from youtube and returns it
pub async fn get_source<'a>(
    ctx: &Context,
    msg: &Message,
    track_name: String,
) -> CommandResult<Option<Restartable>> {
    let source: Restartable;

    if track_name.starts_with("https") || track_name.starts_with("http") {
        source = match Restartable::ytdl_search(track_name, true).await {
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
        source = match Restartable::ytdl_search(&track_name, true).await {
            //gets the track from youtube by the song's name
            Ok(input) => input,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, format!("❌ | Ha ocurrido un error al buscar '{}', puede ser por no ser encontrada o por restricción de edad", track_name))
                    .await?;

                return Ok(None);
            }
        };
    }

    Ok(Some(source))
}
