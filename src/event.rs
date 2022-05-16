use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event {
    #[serde(rename = "gameInfo")]
    pub game_info: GameInfo,
}

#[derive(Deserialize)]
pub struct GameInfo {
    #[serde(rename = "gameDuration")]
    pub game_duration: Option<String>,
}
