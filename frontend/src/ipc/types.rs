use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub method: IpcMethod,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IpcMethod {
    InitAuth,
    CheckAuth,

    Search,
    GetSearchSuggestions,

    GetLibraryPlaylists,
    GetPlaylist,
    GetLibraryAlbums,
    GetLibrarySongs,
    GetLibraryArtists,

    GetArtist,
    GetAlbum,
    GetSong,
    GetLyrics,
    GetWatchPlaylist,

    RateSong,
}
