use crate::event::Event;
use crate::scoreboard::Scoreboard;
use std::ops::Add;
use time::macros::offset;
use time::{format_description, Duration, PrimitiveDateTime, Time};

mod event;
mod scoreboard;

pub async fn get_game_times(date: &str) -> Vec<String> {
    let json_raw = reqwest::get(format!(
        "https://site.api.espn.com/apis/site/v2/sports/baseball/mlb/scoreboard?dates={}",
        date
    )).await.unwrap().text().await.unwrap();

    let mut output = Vec::new();

    let scoreboard = serde_json::from_str::<Scoreboard>(&json_raw).unwrap();
    let date_format = format_description::parse("[year]-[month]-[day]T[hour]:[minute]Z").unwrap();
    let time_format = format_description::parse("[hour padding:space]:[minute]").unwrap();
    for event_entry in scoreboard.events {
        let start_time = PrimitiveDateTime::parse(&event_entry.date, &date_format)
            .unwrap()
            .assume_offset(offset!(UTC))
            .to_offset(offset!(-4));
        let mut end_time = start_time;
        let json_raw = reqwest::get(format!(
            "https://site.api.espn.com/apis/site/v2/sports/baseball/mlb/summary?event={}",
            event_entry.id
        )).await.unwrap().text().await.unwrap();
        let event = serde_json::from_str::<Event>(&json_raw).unwrap();
        if let Some(duration) = event.game_info.game_duration {
            let game_duration = format!("{:>5}", duration.trim());
            let game_time = Time::parse(&game_duration, &time_format).unwrap();
            end_time = end_time
                .add(Duration::minutes(game_time.minute() as i64))
                .add(Duration::hours(game_time.hour() as i64));
            output.push(format!("{},{},{},{}", event_entry.name,
                                format_args!("{}", start_time.date()),
                                format_args!("{:0>2}:{:0>2} EDT", start_time.hour(), start_time.minute()),
                                format_args!("{:0>2}:{:0>2} EDT", end_time.hour(), end_time.minute())));
        }
    }
    output
}
