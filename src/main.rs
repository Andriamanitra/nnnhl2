mod api_client;
mod api_types;

use api_client::CachedFetch;
use chrono::Duration;
use maud::{DOCTYPE, Markup, PreEscaped, html};
use std::sync::Arc;

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
            li.game.(game.game_state) {
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

fn nav() -> Markup {
    html! {
        nav {
            ul {
                li { a href="schedule" { "schedule" } }
                li { a href="standings" { "standings" } }
            }
        }
    }
}

fn schedule_page(schedule: api_types::Schedule) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                style {
                    (include_str!("../css/reset.css"))
                    (include_str!("../css/layout.css"))
                    (include_str!("../css/styles.css"))
                    (include_str!("../css/schedule.css"))
                }
                title { "No-Nonsense NHL schedule" }
            }
            body {
                (nav())
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
    }
}

fn standings_page(standings: api_types::Standings) -> Markup {
    fn last10(team: &api_types::TeamStanding) -> Markup {
        html! {
            (team.l10regulation_plus_ot_wins)
            "-"
            (team.l10losses)
            "-"
            (team.l10ot_losses)
        }
    }

    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                style {
                    (include_str!("../css/reset.css"))
                    (include_str!("../css/layout.css"))
                    (include_str!("../css/styles.css"))
                    (include_str!("../css/standings.css"))
                }
                title { "No-Nonsense NHL standings" }
            }
            body {
                (nav())
                h1 { "NHL Standings" }
                ol {
                    @for team in standings.standings {
                        li."team-standing" {
                            span.(team.team_abbrev) { (team.team_abbrev) }
                            span { (team.games_played) "G" }
                            span { (format!("{:.3}", team.point_pctg)) }
                            span { (team.points) "p" }
                            span { (format!("{:+}", team.goal_differential)) }
                            span { "(" (last10(&team)) ")" }
                            span { (team.streak_code) (team.streak_count) }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let host = std::env::var("HOST").unwrap_or("127.0.0.1:8002".to_string());

    let schedule_fetcher =
        Arc::new(CachedFetch::new(api_client::fetch_schedule).set_cache_ttl(Duration::minutes(5)));

    let standings_fetcher =
        Arc::new(CachedFetch::new(api_client::fetch_standings).set_cache_ttl(Duration::hours(1)));

    println!("Listening on http://{host}/");
    rouille::start_server(host, move |req| {
        println!("{} {}", req.method(), req.url());
        match req.url().as_str() {
            "/" | "/schedule" => match schedule_fetcher.get() {
                Ok(schedule) => rouille::Response::html(schedule_page(schedule)),
                Err(e) => {
                    eprintln!("{e}");
                    rouille::Response::text("failed to get schedule from nhl api")
                        .with_status_code(500)
                }
            },
            "/standings" => match standings_fetcher.get() {
                Ok(standings) => rouille::Response::html(standings_page(standings)),
                Err(e) => {
                    eprintln!("{e}");
                    rouille::Response::text("failed to get standings from nhl api")
                        .with_status_code(500)
                }
            },
            _ => rouille::Response::text("404 Not found").with_status_code(404),
        }
    });
}
