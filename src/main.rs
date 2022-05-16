use dioxus::core::to_owned;
use dioxus::prelude::*;
use futures::StreamExt;
use mlb_time_puller::get_game_times;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let table_rows: &UseState<Option<Vec<String>>> = use_state(&cx, || None);
    let date = use_state(&cx, || "2022-05-15".to_string());

    let text_routine = use_coroutine(&cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![table_rows];
        async move  {
            if let Some(date) = rx.next().await {
                table_rows.set(Some(get_game_times(&date.replace("-", "")).await));
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
            onclick: move |_| text_routine.send(date.get().to_string()),
            "Load Data"
        }
        input {
            r#type: "date",
            value: "{date}",
            oninput: move |evt| date.set(evt.value.clone())
        }

        table
    })
}
