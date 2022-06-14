use serde::Deserialize;

#[derive(Deserialize)]
pub struct Schedule {
    pub dates: Vec<Date>,
}

#[derive(Deserialize)]
pub struct Date {
    pub games: Vec<Game>,
}

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "gamePk")]
    pub game_pk: usize,
    pub status: Status,
    pub teams: Teams,
    pub broadcasts: Vec<Broadcast>,
}

#[derive(Deserialize)]
pub struct Broadcast {
    pub id: u16,
    pub name: String,
    #[serde(rename = "type")]
    pub broadcast_type: String,
    #[serde(rename = "homeAway")]
    pub home_away: String,
}

#[derive(Deserialize)]
pub struct Status {
    #[serde(rename = "detailedState")]
    pub detailed_state: String,
}

#[derive(Deserialize)]
pub struct Teams {
    pub away: Away,
    pub home: Home,
}

#[derive(Deserialize)]
pub struct Away {
    pub team: Team,
}

#[derive(Deserialize)]
pub struct Home {
    pub team: Team,
}

#[derive(Deserialize)]
pub struct Team {
    pub id: u16,
    pub name: String,
}
