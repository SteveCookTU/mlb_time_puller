use crate::event::Event;
use crate::scoreboard::Scoreboard;
use std::ops::Add;
use time::macros::offset;
use time::{format_description, Duration, PrimitiveDateTime, Time, UtcOffset};
use crate::teams::Team;
use crate::timezone::Timezone;

mod event;
mod scoreboard;
pub mod timezone;
pub mod teams;

pub async fn get_game_times(date: &str, timezone: Timezone, team: Team) -> Vec<String> {
    let json_raw = reqwest::get(format!(
        "https://site.api.espn.com/apis/site/v2/sports/baseball/mlb/scoreboard?dates={}",
        date
    )).await.unwrap().text().await.unwrap();

    let mut output = Vec::new();

    let scoreboard = serde_json::from_str::<Scoreboard>(&json_raw).unwrap();
    let date_format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]Z").unwrap();
    let time_format = format_description::parse("[hour padding:space]:[minute]").unwrap();
    for event_entry in scoreboard.events {
        let team1: Team = event_entry.competitions[0].competitors[0].team.id.parse::<u8>().unwrap().try_into().unwrap();
        let team2: Team = event_entry.competitions[0].competitors[1].team.id.parse::<u8>().unwrap().try_into().unwrap();
        if team == Team::All || team == team1 || team == team2 {
            let start_time = PrimitiveDateTime::parse(&event_entry.date, &date_format)
                .unwrap()
                .assume_offset(offset!(UTC))
                .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
            let mut end_time_str = "N/A".to_string();
            let json_raw = reqwest::get(format!(
                "https://site.api.espn.com/apis/site/v2/sports/baseball/mlb/summary?event={}",
                event_entry.id
            )).await.unwrap().text().await.unwrap();

            let event = serde_json::from_str::<Event>(&json_raw).unwrap();
            if let Some(duration) = event.game_info.game_duration {
                let game_duration = format!("{:>5}", duration.trim());
                let game_time = Time::parse(&game_duration, &time_format).unwrap();
                let end_time = start_time
                    .add(Duration::minutes(game_time.minute() as i64))
                    .add(Duration::hours(game_time.hour() as i64));
                end_time_str = format!("{:0>2}:{:0>2} {}", end_time.hour(), end_time.minute(), timezone);
            }
            output.push(format!("{},{},{},{}", event_entry.name,
                                format_args!("{}", start_time.date()),
                                format_args!("{:0>2}:{:0>2} {}", start_time.hour(), start_time.minute(), timezone),
                                end_time_str));
        }

    }
    output
}
