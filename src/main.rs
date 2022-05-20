use dioxus::core::to_owned;
use dioxus::prelude::*;
use futures::StreamExt;
use mlb_time_puller::get_mlb_times;
use mlb_time_puller::teams::{Team, TEAMS};
use mlb_time_puller::timezone::Timezone;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let table_rows: &UseState<Option<Vec<String>>> = use_state(&cx, || None);
    let date = use_state(&cx, || "2022-05-15".to_string());
    let loading = use_state(&cx, || false);
    let timezone = use_state(&cx, || Timezone::EDT);
    let team = use_state(&cx, || Team::All);

    let text_routine = use_coroutine(
        &cx,
        |mut rx: UnboundedReceiver<(String, Timezone, Team)>| {
            to_owned![table_rows, loading];
            async move {
                while let Some((date, timezone, team)) = rx.next().await {
                    table_rows.set(Some(get_mlb_times(&date, timezone, team).await));
                    loading.set(false);
                }
            }
        },
    );

    let table = if let Some(rows) = table_rows.get() {
        let rows = rows.iter().map(|row| {
            let mut row = row.split(',');
            let title = row.next().unwrap();
            let date = row.next().unwrap();
            let venue_start = row.next().unwrap();
            let venue_end = row.next().unwrap();
            let duration = row.next().unwrap();
            let delay = row.next().unwrap();
            let start = row.next().unwrap();
            let end = row.next().unwrap();
            let broadcasts = row.next().unwrap().replace('.', ",");
            rsx! {
                tr {
                    td {
                        "{title}"
                    }
                    td {
                        class: "center",
                        "{date}"
                    }
                    td {
                        class: "center",
                        "{venue_start}"
                    }
                    td {
                        class: "center",
                        "{venue_end}"
                    }
                    td {
                        class: "center",
                        "{duration}"
                    }
                    td {
                        class: "center",
                        "{delay}"
                    }
                    td {
                        class: "center",
                        "{start}"
                    }
                    td {
                        class: "center",
                        "{end}"
                    }
                    td {
                        class: "center",
                        "{broadcasts}"
                    }
                }
            }
        });
        rsx! {
            style { ["td { padding: 3px 10px;} .center { text-align: center; } th { padding: 0px 15px; }"] }
            table {
                tr {
                    th {
                        "Game"
                    }
                    th {
                        "Date"
                    }
                    th {
                        "Venue Start Time"
                    }
                    th {
                        "Venue End Time"
                    }
                    th {
                        "Game Duration"
                    }
                    th {
                        "Delay Duration"
                    }
                    th {
                        "Converted Start Time"
                    }
                    th {
                        "Converted End Time"
                    }
                    th {
                        "Broadcasts"
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

    cx.render(rsx! {
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
            "Convert to Timezone:",
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
                    let i = evt.value.parse::<u16>().unwrap();
                    team.set(Team::try_from(i).unwrap());
                },
                TEAMS.iter().map(|&team| {
                    let value: u16 = team.into();
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
