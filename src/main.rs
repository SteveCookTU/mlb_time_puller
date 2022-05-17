use dioxus::core::to_owned;
use dioxus::prelude::*;
use futures::StreamExt;
use mlb_time_puller::get_game_times;
use mlb_time_puller::timezone::Timezone;
use mlb_time_puller::teams::{Team, TEAMS};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let table_rows: &UseState<Option<Vec<String>>> = use_state(&cx, || None);
    let date = use_state(&cx, || "2022-05-15".to_string());
    let loading = use_state(&cx, || false);
    let timezone = use_state(&cx, || Timezone::EDT);
    let team = use_state(&cx, || Team::All);

    let text_routine = use_coroutine(&cx, |mut rx: UnboundedReceiver<(String, Timezone, Team)>| {
        to_owned![table_rows, loading];
        async move  {
            while let Some((date, timezone, team)) = rx.next().await {
                table_rows.set(Some(get_game_times(&date.replace("-", ""), timezone, team).await));
                loading.set(false);
            }
        }
    });

    let table = if let Some(rows) = table_rows.get() {
        let rows = rows.iter().map(|row| {
            let mut row = row.split(",");
            let game = row.next().unwrap();
            let date = row.next().unwrap();
            let start = row.next().unwrap();
            let end = row.next().unwrap();
            rsx! {
                tr {
                    td {
                        "{game}"
                    }
                    td {
                        "{date}"
                    }
                    td {
                        "{start}"
                    }
                    td {
                        "{end}"
                    }
                }
            }
        });
        rsx! {
            style { ["td { padding: 3px 15px;}"] }
            table {
                tr {
                    th {
                        "Game"
                    }
                    th {
                        "Date"
                    }
                    th {
                        "Start Time"
                    }
                    th {
                        "End Time"
                    }
                }
                rows
            }
        }
    } else {
        rsx! {
            div {
                "Search for results"
            }
        }
    };


    cx.render(rsx!{
        button {
            style: "margin-right: 5px;",
            onclick: move |_| {
                loading.set(true);
                text_routine.send((date.get().to_string(), *timezone.get(), *team.get()))
            },
            disabled: "{loading}",
            "Load Data"
        }
        input {
            style: "margin-right: 5px;",
            r#type: "date",
            value: "{date}",
            oninput: move |evt| date.set(evt.value.clone())
        }
        p {
            "Timezone:",
            select {
                style: "margin-left: 5px;",
                onchange: move |evt| {
                    let i = evt.value.parse::<i8>().unwrap();
                    timezone.set(Timezone::try_from(i).unwrap());
                },
                option {
                    value: "-4",
                    "EDT"
                }
                option {
                    value: "-5",
                    "CDT"
                }
                option {
                    value: "-6",
                    "MDT"
                }
                option {
                    value: "-7",
                    "PDT"
                }
            }
        }
        p {
            "Team:",
            select {
                style: "margin-left: 5px;",
                onchange: move |evt| {
                    let i = evt.value.parse::<u8>().unwrap();
                    team.set(Team::try_from(i).unwrap());
                },
                TEAMS.iter().map(|&team| {
                    let value: u8 = team.into();
                    rsx!(
                        option {
                            value: "{value}",
                            "{team}"
                        }
                    )
                })
            }
        }
        table
    })
}
