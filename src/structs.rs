#[derive(Debug, Deserialize, Serialize)]
pub struct Team {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Standing {
    pub team: Team,
    pub position: i32,
    pub played_games: i32,
    pub points: i32,
    pub goal_difference: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Competition {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Split {
    pub table: Vec<Standing>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub home_team: Option<i32>,
    pub away_team: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub full_time: Result,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub current_matchday: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub home_team: Team,
    pub away_team: Team,
    pub score: Score,
    pub utc_date: String,
    pub matchday: i32,
    pub season: Season,
    pub competition: Option<Competition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct League {
    pub competition: Competition,
    pub standings: Vec<Split>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fixtures {
    pub competition: Option<Competition>,
    pub matches: Vec<Match>,
}
