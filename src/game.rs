use serde::Deserialize;

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Deserialize)]
pub struct GameData {
    pub venue: Venue,
    #[serde(rename = "gameInfo")]
    pub game_info: GameInfo,
}

#[derive(Deserialize)]
pub struct Venue {
    #[serde(rename = "timeZone")]
    pub time_zone: TimeZone,
}

#[derive(Deserialize)]
pub struct TimeZone {
    pub offset: i8,
    pub tz: String,
}

#[derive(Deserialize)]
pub struct GameInfo {
    #[serde(rename = "firstPitch")]
    pub first_pitch: String,
    #[serde(rename = "gameDurationMinutes")]
    pub game_duration_minutes: i64,
    #[serde(rename = "delayDurationMinutes")]
    pub delay_duration_minutes: Option<i64>,
}
