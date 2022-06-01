use dotenv;
use rand::Rng;
use rspotify::{
    model::{playlist::FullPlaylist, search::SearchResult, PlayableItem, SearchType},
    prelude::*,
    ClientCredsSpotify, ClientResult, Credentials,
};
use serenity::{client::Context, model::channel::Message, utils::Colour};

pub async fn get_play_list(
    ctx: &Context,
    msg: &Message,
    play_list_name: &str,
) -> ClientResult<Option<FullPlaylist>> {
    // You can use any logger for debugging.
    dotenv::dotenv().unwrap();

    let creds = Credentials::from_env().unwrap();
    let mut spotify = ClientCredsSpotify::new(creds);

    // Obtainin the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token()?;

    let play_list_searched = spotify.search(
        play_list_name,
        &SearchType::Playlist,
        None,
        None,
        Some(1),
        None,
    )?;

    if let SearchResult::Playlists(list) = play_list_searched {
        let play_list = match spotify.playlist(&list.items[0].id, None, None) {
            Ok(play_list) => play_list,
            Err(_err) => {
                msg.channel_id
                    .say(&ctx.http, "❌ | No se ha podido encontrar esa play list")
                    .await
                    .unwrap();

                return Ok(None);
            }
        };

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field("🎸 PlayList:", &play_list.name, true)
                        .fields(
                            play_list
                                .tracks
                                .items
                                .iter()
                                .map(|track| -> (String, String, bool) {
                                    if let PlayableItem::Track(t) = track.track.as_ref().unwrap() {
                                        (".".to_string(), t.name.to_owned(), false)
                                    } else {
                                        (".".to_string(), "".to_string(), true)
                                    }
                                })
                                .collect::<Vec<(String, String, bool)>>(),
                        )
                        .colour(Colour::from_rgb(
                            rand::thread_rng().gen_range(0..255),
                            rand::thread_rng().gen_range(0..255),
                            rand::thread_rng().gen_range(0..255),
                        ))
                })
            })
            .await
            .expect("Coudln't send the message");

        return Ok(Some(play_list));
    }

    Ok(None)
}
