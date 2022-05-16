use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scoreboard {
    pub events: Vec<EventEntry>,
}

#[derive(Deserialize)]
pub struct EventEntry {
    pub id: String,
    pub uid: String,
    pub date: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
}
