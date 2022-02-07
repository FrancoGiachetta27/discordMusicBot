use rspotify::{
    model::{
        SearchType,
        PlayableItem,
        search::SearchResult,
        playlist::FullPlaylist,
    },
    prelude::*,
    ClientCredsSpotify,
    ClientResult,
    Credentials
};
use rand::Rng;
use serenity::{
    model::{
        channel::Message
    },
    client::Context,
    utils::Colour,
};
use dotenv;

pub async fn getPlayList(ctx:&Context, msg:&Message, playListName: &str) -> ClientResult<Option<FullPlaylist>> {
    // You can use any logger for debugging.
    dotenv::dotenv().unwrap();

    let creds = Credentials::from_env().unwrap();
    let mut spotify = ClientCredsSpotify::new(creds);

    // Obtainin the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token()?;

    let playListSearched = spotify.search(playListName, &SearchType::Playlist, None, None, Some(1), None)?;
    let mut playListSongs:Vec<(String,String,bool)> = Vec::new();

    if let SearchResult::Playlists(list) = playListSearched {
        let playList = match spotify.playlist(&list.items[0].id, None, None) {
            Ok(playList) => playList,
            Err(why) => {
                msg.channel_id.say(&ctx.http,"❌ | No se ha podido encontrar esa play list").await.unwrap();

                return Ok(None);
            }
        };

        for track in playList.tracks.items.iter() {
            match track.track.as_ref().unwrap() {
                PlayableItem::Track(t) => {
                    playListSongs.push((".".to_string(),t.name.to_owned(),true));
                }
                _ => { return Ok(None); }
            }
        }

        msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.field("🎸 PlayList:", &playList.name,true)
                    .fields(
                        playListSongs
                    )
                    .colour(Colour::from_rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255)))
                })
        }).await.expect("Coudln't send the message");

        return Ok(Some(playList));
    }

    Ok(None)
}