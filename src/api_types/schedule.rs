use crate::api_types::*;

/// Response type for the following end points:
/// * `/v1/schedule/now`
/// * `/v1/schedule/SEASON_ID`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "nextStartDate")]
    pub next_start_date: NaiveDate,
    #[serde(rename = "previousStartDate")]
    pub previous_start_date: NaiveDate,
    #[serde(rename = "gameWeek")]
    pub game_week: Vec<GameWeek>,
    #[serde(rename = "preSeasonStartDate")]
    pub pre_season_start_date: NaiveDate,
    #[serde(rename = "regularSeasonStartDate")]
    pub regular_season_start_date: NaiveDate,
    #[serde(rename = "regularSeasonEndDate")]
    pub regular_season_end_date: NaiveDate,
    #[serde(rename = "playoffEndDate")]
    pub playoff_end_date: NaiveDate,
    #[serde(rename = "numberOfGames")]
    pub number_of_games: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameWeek {
    pub date: NaiveDate,
    #[serde(rename = "dayAbbrev")]
    pub day_abbrev: String,
    #[serde(rename = "numberOfGames")]
    pub number_of_games: usize,
    pub games: Vec<Game>,
}

/// Response type for the following end points:
/// * `/v1/club-schedule-season/TEAM_ABBR/now`
/// * `/v1/club-schedule-season/TEAM_ABBR/SEASON_ID`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClubScheduleSeason {
    #[serde(rename = "previousSeason")]
    pub previous_season: SeasonId,
    #[serde(rename = "currentSeason")]
    pub current_season: SeasonId,
    #[serde(rename = "clubTimezone")]
    pub club_timezone: String,
    #[serde(rename = "clubUTCOffset")]
    pub club_utcoffset: String,
    pub games: Vec<Game>,
}

/// Response type for the following end points:
/// * `/v1/club-schedule/TEAM_ABBR/month/now`
/// * `/v1/club-schedule/TEAM_ABBR/month/YYYY-MM`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClubScheduleMonth {
    #[serde(rename = "previousMonth")]
    pub previous_month: String,
    #[serde(rename = "currentMonth")]
    pub current_month: String,
    #[serde(rename = "nextMonth")]
    pub next_month: String,
    #[serde(rename = "calendarUrl")]
    pub calendar_url: String,
    #[serde(rename = "clubTimezone")]
    pub club_timezone: String,
    #[serde(rename = "clubUTCOffset")]
    pub club_utcoffset: String,
    pub games: Vec<Game>,
}

/// Response type for the following end points:
/// * `/v1/club-schedule/TEAM_ABBR/week/now`
/// * `/v1/club-schedule/TEAM_ABBR/week/YYYY-MM-DD`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClubScheduleWeek {
    #[serde(rename = "previousStartDate")]
    pub previous_start_date: NaiveDate,
    #[serde(rename = "nextStartDate")]
    pub next_start_date: NaiveDate,
    #[serde(rename = "calendarUrl")]
    pub calendar_url: String,
    #[serde(rename = "clubTimezone")]
    pub club_timezone: String,
    #[serde(rename = "clubUTCOffset")]
    pub club_utcoffset: String,
    pub games: Vec<Game>,
}
