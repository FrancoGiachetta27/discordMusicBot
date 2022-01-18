use rspotify::{
    model::{
        SearchType,
        PlayableItem,
        PlaylistItem,
        track::FullTrack,
        search::SearchResult,
        enums::{
            misc::Market,
            country::Country
        },
        playlist::FullPlaylist,
    },
    prelude::*,
    ClientCredsSpotify,
    ClientResult,
    Credentials
};
use serenity:: {
    model::{
        channel::{Message}, 
        gateway::{Ready},
    },
    client::Context,
};
use dotenv;

pub async fn getPlayList(ctx:&Context, msg:&Message, playListName: &str) -> ClientResult<Option<FullPlaylist>> {
    // You can use any logger for debugging.
    dotenv::dotenv().unwrap();

    let creds = Credentials::from_env().unwrap();
    let mut spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token. Requires to be mutable because the internal
    // token will be modified. We don't need OAuth for this specific endpoint,
    // so `...` is used instead of `prompt_for_user_token`.
    spotify.request_token()?;

    let playListSearched = spotify.search(playListName, &SearchType::Playlist, None, None, Some(1), None)?;

    if let SearchResult::Playlists(list) = playListSearched {
        let playList = match spotify.playlist(&list.items[0].id, None, None) {
            Ok(playList) => playList,
            Err(why) => {
                msg.channel_id.say(&ctx.http,"❌ | No se ha podido encontrar esa play list").await.unwrap();
                
                return Ok(None);
            }
        };

        return Ok(Some(playList));
    }

    Ok(None)
}
