use chrono::prelude::*;
use chrono::{Duration, Local};
use prettytable;
use reqwest;
use reqwest::header::Headers;
use serde_json;

use structs::Competition;
use structs::Fixtures;
use structs::League;
use structs::Match;
use structs::Score;
use structs::Standing;
use utils::{format_date, read_file};

fn construct_headers() -> Headers {
    let mut headers = Headers::new();
    let api_token = read_file("api_token".to_string());
    headers.set_raw("x-auth-token", api_token);
    headers
}

pub fn get_fixtures(league: &String) -> (Option<Competition>, Vec<Match>) {
    let client = reqwest::Client::new();
    let endpoint = String::from("http://api.football-data.org/v2/competitions/");
    let suffix = String::from("/matches");
    let joined: String = endpoint + &league + &suffix;

    let request = client.get(&joined).headers(construct_headers()).send();
    let mut resp = request.unwrap();
    assert!(resp.status().is_success());

    let string = resp.text().unwrap();
    let league: Fixtures = serde_json::from_str(&string).unwrap();

    let mut filtered = Vec::new();
    for m in league.matches {
        if m.season.current_matchday == m.matchday {
            filtered.push(m)
        }
    }

    (league.competition, filtered)
}

pub fn print_fixtures(league: Option<Competition>, fixtures: Vec<Match>) {
    let mut table = prettytable::Table::new();

    table.add_row(row![league.unwrap().name]);
    table.add_row(row!["Date", "Home", "Score", "Away"]);
    for f in fixtures.iter() {
        table.add_row(row![
            format_date(&f.utc_date),
            f.home_team.name,
            construct_score(&f.score),
            f.away_team.name
        ]);
    }

    table.printstd();
}

pub fn get_team_fixtures(team: &String) -> (Vec<Match>) {
    let now = Local::now();
    let one_month = now + Duration::weeks(4);
    let from_date = [
        now.year().to_string(),
        format!("{:0>2}", now.month().to_string()),
        format!("{:0>2}", now.day().to_string()),
    ];
    let to_date = [
        one_month.year().to_string(),
        format!("{:0>2}", one_month.month().to_string()),
        format!("{:0>2}", one_month.day().to_string()),
    ];
    let from_joined = from_date.join("-");
    let to_joined = to_date.join("-");
    let endpoint = String::from("http://api.football-data.org/v2/teams/");
    let suffix = String::from("/matches");
    let joined: String =
        endpoint + &team + &suffix + "?dateFrom=" + &from_joined + "&dateTo=" + &to_joined;

    let client = reqwest::Client::new();
    let request = client.get(&joined).headers(construct_headers()).send();
    let mut resp = request.unwrap();
    assert!(resp.status().is_success());

    let string = resp.text().unwrap();
    let league: Fixtures = serde_json::from_str(&string).unwrap();

    league.matches
}

pub fn print_team_fixtures(fixtures: Vec<Match>) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Date", "Competition", "Home", "Score", "Away"]);
    for f in fixtures.into_iter() {
        table.add_row(row![
            format_date(&f.utc_date),
            f.competition.unwrap().name,
            f.home_team.name,
            construct_score(&f.score),
            f.away_team.name
        ]);
    }

    table.printstd();
}

pub fn get_league(league: &String) -> League {
    let client = reqwest::Client::new();
    let endpoint = String::from("http://api.football-data.org/v2/competitions/");
    let suffix = String::from("/standings");
    let joined: String = endpoint + &league + &suffix;
    let request = client.get(&joined).headers(construct_headers()).send();

    let mut resp = request.unwrap();
    assert!(resp.status().is_success());

    let json = resp.text().unwrap();

    serde_json::from_str(&json).unwrap()
}

pub fn print_league_table(league: League) {
    let mut table = prettytable::Table::new();

    table.add_row(row![league.competition.name]);
    table.add_row(row![
        "Position",
        "Team",
        "Games Played",
        "Goal Difference",
        "Points"
    ]);
    let league_table: &Vec<Standing> = &league.standings[0].table;
    for team in league_table.iter() {
        table.add_row(row![
            team.position,
            team.team.name,
            team.played_games,
            team.goal_difference,
            team.points
        ]);
    }

    table.printstd();
}

fn unwrap_score(score: Option<i32>) -> (String) {
    if score.is_some() {
        score.unwrap().to_string()
    } else {
        String::from("N/A")
    }
}

pub fn construct_score(score: &Score) -> (String) {
    let home_score = unwrap_score(score.full_time.home_team);
    let away_score = unwrap_score(score.full_time.away_team);
    home_score + " - " + &away_score
}
