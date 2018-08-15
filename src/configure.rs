use dialoguer::Input;
use dialoguer::Select;

use requests::get_league;
use structs::Standing;

pub fn select_league() -> (String, i32) {
    let codes = vec![2021, 2014, 2019, 2002, 2015];
    let names = vec![
        "Premier League",
        "La Liga",
        "Serie A",
        "Bundesliga",
        "Ligue 1",
    ];

    let index = Select::new()
        .default(0)
        .items(&names[..])
        .interact()
        .unwrap();

    (names[index].to_string(), codes[index])
}

pub fn select_team(league_code: i32) -> (String, i32) {
    let res = &get_league(&league_code.to_string());
    let standings: &Vec<Standing> = &res.standings[0].table;
    let mut codes = Vec::new();
    let mut names = Vec::new();
    for ref standing in standings.into_iter() {
        codes.push(standing.team.id);
        names.push(standing.team.name.as_ref());
    }

    let index = Select::new()
        .default(0)
        .items(&names[..])
        .interact()
        .unwrap();

    (names[index].to_string(), codes[index])
}

pub fn input_token() -> String {
    let token = Input::new(
        "Insert your API token (available free from https://www.football-data.org/)",
    ).interact()
        .unwrap();
    token
}
