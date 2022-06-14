use crate::game::Game;
use crate::schedule::Schedule;
use crate::teams::Team;
use crate::timezone::Timezone;
use std::ops::Add;
use time::format_description::well_known;
use time::{Duration, OffsetDateTime, UtcOffset};

mod game;
mod schedule;
pub mod teams;
pub mod timezone;

pub async fn get_mlb_times(date: &str, timezone: Timezone, team: Team) -> Vec<String> {
    let json_raw = reqwest::get(format!(
        "https://statsapi.mlb.com/api/v1/schedule?language=en&sportId=1&date={}&hydrate=game,broadcasts",
        date
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let mut output = Vec::new();

    let schedule = serde_json::from_str::<Schedule>(&json_raw).unwrap();
    for schedule_date in schedule.dates {
        for schedule_game in schedule_date.games {
            let away: Team = schedule_game.teams.away.team.id.try_into().unwrap();
            let home: Team = schedule_game.teams.home.team.id.try_into().unwrap();
            if team == Team::All || team == away || team == home {
                if schedule_game.status.detailed_state != *"Final" {
                    output.push(format!(
                        "{} at {},{},{},{},{},{},{},{},{}",
                        away,
                        home,
                        schedule_game.status.detailed_state,
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                        " ",
                    ));
                    continue;
                }

                let json_raw = reqwest::get(format!(
                    "https://statsapi.mlb.com/api/v1.1/game/{}/feed/live",
                    schedule_game.game_pk
                ))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                if let Ok(game) = serde_json::from_str::<Game>(&json_raw) {
                    let start_time = OffsetDateTime::parse(
                        &game.game_data.game_info.first_pitch,
                        &well_known::Rfc3339,
                    )
                    .unwrap()
                    .to_offset(UtcOffset::from_hms(timezone.into(), 0, 0).unwrap());
                    if start_time.date().to_string() != *date {
                        continue;
                    }

                    let delay_time: Duration = {
                        let play = &game.live_data.plays.all_plays[0];
                        let mut duration_time = Duration::minutes(
                            game.game_data
                                .game_info
                                .delay_duration_minutes
                                .unwrap_or_default(),
                        );
                        for play_event in play.play_events.iter() {
                            if play_event
                                .details
                                .description
                                .as_ref()
                                .unwrap()
                                .to_lowercase()
                                .contains("delayed start")
                            {
                                duration_time = Duration::minutes(0);
                                break;
                            }
                        }
                        if duration_time.is_zero() {
                            for all_play in game.live_data.plays.all_plays.iter().skip(1) {
                                for play_event in all_play.play_events.iter() {
                                    if let Some(description) =
                                        play_event.details.description.as_ref()
                                    {
                                        if description.to_lowercase().contains("delayed") {
                                            if let Some(end_time) = play_event.end_time.as_ref() {
                                                let start_time = OffsetDateTime::parse(
                                                    &play_event.start_time,
                                                    &well_known::Rfc3339,
                                                )
                                                .unwrap();

                                                let end_time = OffsetDateTime::parse(
                                                    end_time,
                                                    &well_known::Rfc3339,
                                                )
                                                .unwrap();

                                                let duration = end_time - start_time;
                                                duration_time += duration;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        duration_time
                    };

                    let end_time = start_time
                        .add(Duration::minutes(
                            game.game_data.game_info.game_duration_minutes,
                        ))
                        .add(delay_time);
                    let game_duration =
                        Duration::minutes(game.game_data.game_info.game_duration_minutes);
                    let delay_duration = Duration::minutes(
                        game.game_data
                            .game_info
                            .delay_duration_minutes
                            .unwrap_or_default(),
                    );

                    let pre_game_delay = delay_duration - delay_time;

                    let venue_start_time = start_time.to_offset(
                        UtcOffset::from_hms(game.game_data.venue.time_zone.offset, 0, 0).unwrap(),
                    );
                    let venue_end_time = end_time.to_offset(
                        UtcOffset::from_hms(game.game_data.venue.time_zone.offset, 0, 0).unwrap(),
                    );
                    let broadcasts = schedule_game
                        .broadcasts
                        .iter()
                        .filter_map(|b| {
                            if b.broadcast_type == *"TV" {
                                Some(format!(
                                    "{} ({})",
                                    b.name.replace("(out-of-market only)", ""),
                                    b.home_away.clone()
                                ))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(". ");
                    output.push(format!(
                        "{} at {},{},{},{},{},{},{},{},{},{}",
                        away,
                        home,
                        format_args!("{}", start_time.date()),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            venue_start_time.hour(),
                            venue_start_time.minute(),
                            game.game_data.venue.time_zone.tz
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            venue_end_time.hour(),
                            venue_end_time.minute(),
                            game.game_data.venue.time_zone.tz
                        ),
                        format_args!(
                            "{}:{:0>2}",
                            game_duration.whole_hours(),
                            game_duration.whole_minutes() % 60
                        ),
                        format_args!(
                            "{}:{:0>2}",
                            pre_game_delay.whole_hours(),
                            pre_game_delay.whole_minutes() % 60,
                        ),
                        format_args!(
                            "{}:{:0>2}",
                            delay_time.whole_hours(),
                            delay_time.whole_minutes() % 60,
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            start_time.hour(),
                            start_time.minute(),
                            timezone
                        ),
                        format_args!(
                            "{:0>2}:{:0>2} {}",
                            end_time.hour(),
                            end_time.minute(),
                            timezone
                        ),
                        broadcasts
                    ));
                }
            }
        }
    }
    output
}
