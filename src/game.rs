use serde::Deserialize;

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "gameData")]
    pub game_data: GameData,
    #[serde(rename = "liveData")]
    pub live_data: LiveData,
}

#[derive(Deserialize)]
pub struct GameData {
    pub venue: Venue,
    #[serde(rename = "gameInfo")]
    pub game_info: GameInfo,
}

#[derive(Deserialize)]
pub struct LiveData {
    pub plays: Plays,
}

#[derive(Deserialize)]
pub struct Plays {
    #[serde(rename = "allPlays")]
    pub all_plays: Vec<Play>
}

#[derive(Deserialize)]
pub struct Play {
    #[serde(rename = "playEvents")]
    pub play_events: Vec<PlayEvent>
}

#[derive(Deserialize)]
pub struct PlayEvent {
    pub details: PlayEventDetails,
    #[serde(rename = "startTime")]
    pub start_time: String,
}

#[derive(Deserialize)]
pub struct PlayEventDetails {
    pub description: String,
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
