#[macro_use]
extern crate clap;
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate prettytable;
extern crate dialoguer;

mod configure;
mod requests;
mod structs;
mod utils;

use clap::App;
use dialoguer::Confirmation;
use std::fs::File;

use configure::{input_token, select_league, select_team};
use requests::{
    get_fixtures, get_league, get_team_fixtures, print_fixtures, print_league_table,
    print_team_fixtures,
};
use utils::{read_file, write_file};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    if let Some(_matches) = matches.subcommand_matches("configure") {
        let existing_token = File::open("api_token");
        if existing_token.is_ok() {
            let choice = Confirmation::new(
                "An API token has previously been set, would you like to use that?",
            ).interact();
            if !choice.unwrap() {
                let api_token = input_token();
                let _res = write_file("api_token".to_string(), api_token.to_string());
            }
        } else {
            let api_token = input_token();
            let _res = write_file("api_token".to_string(), api_token.to_string());
        }

        println!("Select your favourite league");
        let (_league_name, league_code) = select_league();
        let _res = write_file("league".to_string(), league_code.to_string());

        println!("Select your favourite team");
        let (_team_name, team_code) = select_team(league_code);
        let _res = write_file("team".to_string(), team_code.to_string());
        return;
    }

    if let Some(matches) = matches.subcommand_matches("table") {
        if matches.is_present("manual") {
            println!("Select league to display");
            let (_league_name, league_code) = select_league();
            let res = get_league(&league_code.to_string());
            print_league_table(res);
        } else {
            let league = read_file("league".to_string());
            let res = get_league(&league);
            print_league_table(res)
        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("fixtures") {
        if matches.is_present("manual") {
            println!("Select league to display");
            let (_league_name, league_code) = select_league();
            let (competition, fixtures) = get_fixtures(&league_code.to_string());
            print_fixtures(competition, fixtures);
        } else if matches.is_present("team") {
            let team = read_file("team".to_string());
            let fixtures = get_team_fixtures(&team.to_string());
            print_team_fixtures(fixtures);
        } else {
            let league = read_file("league".to_string());
            let (competition, fixtures) = get_fixtures(&league.to_string());
            print_fixtures(competition, fixtures);
        }
        return;
    }

    println!("Command required to run program, use --help to see options");
}
