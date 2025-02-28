use maud::{DOCTYPE, Markup, PreEscaped, html};

mod api_types;

fn games2html(games: Vec<api_types::Game>) -> Markup {
    fn game2html(game: api_types::Game) -> Markup {
        let clock = html! {
            time datetime=(game.start_time_utc.to_rfc3339()) {
                (game.start_time_utc.format("%H:%M"))
            }
        };
        let home = html! {
            span.(game.home_team.abbrev) { (game.home_team.abbrev) }
        };
        let away = html! {
            span.(game.away_team.abbrev) { (game.away_team.abbrev) }
        };
        let game_center_link = html! {
            a title="NHL GameCenter" href=(format!("https://nhl.com{}", game.game_center_link)) { "@" }
        };
        let recap_link = match game.condensed_game {
            Some(path) => html! {
                a title="Recap" href=(format!("https://nhl.com{path}")) { "R" }
            },
            None => html! { "|" },
        };
        let score = html! {
            span.score.spoiler[game.game_state == "OFF"] {
                (match (game.home_team.score, game.away_team.score) {
                    (Some(h), Some(a)) => format!("{h}–{a}"),
                    _ => "TBD".to_string()
                })
            }
        };

        html! {
            li.game.playing[game.game_state != "OFF" && game.game_state != "FUT"].(game.game_state) {
                (clock) " " (game_center_link) (home) " - " (away) " " (recap_link) " " (score)
            }
        }
    }

    html! {
        ul.games {
            @for game in games {
                (game2html(game))
            }
        }
    }
}

macro_rules! js_from_file {
    ($src:literal) => {
        html! {
            script {
                (PreEscaped(include_str!($src)))
            }
        }
    };
}

fn get_schedule() -> Result<api_types::Schedule, ureq::Error> {
    let today = chrono::Utc::now();
    let start_date = today
        .checked_sub_days(chrono::Days::new(2))
        .unwrap()
        .format("%Y-%m-%d");
    let nhl_api_url = format!("https://api-web.nhle.com/v1/schedule/{}", start_date);
    println!("Fetching {nhl_api_url} ...");
    ureq::get(nhl_api_url)
        .call()
        .and_then(|mut res| res.body_mut().read_json())
}

fn main() {
    let host = std::env::var("HOST").unwrap_or("127.0.0.1:8002".to_string());
    println!("Listening on http://{host}/");
    rouille::start_server(host, move |req| {
        if req.url().contains("favico") {
            return rouille::Response::empty_404();
        };
        let schedule = match get_schedule() {
            Ok(schedule) => schedule,
            Err(e) => {
                eprintln!("{:?}", e);
                return rouille::Response::text("failed to get schedule from nhl api")
                    .with_status_code(500);
            }
        };
        let markup = html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    style {
                        (include_str!("../css/reset.css"))
                        (include_str!("../css/layout.css"))
                        (include_str!("../css/styles.css"))
                    }
                    title { "No-Nonsense NHL schedule" }
                }
                body {
                    h1 { "NHL Schedule" }

                    @for day in schedule.game_week {
                        h2 {
                            (day.date.format("%a %d.%m."))
                        }
                        (games2html(day.games))
                    }
                    (js_from_file!("../js/time2local.js"))
                    (js_from_file!("../js/spoilers.js"))
                }
            }
        };
        rouille::Response::html(markup.into_string())
    });
}
