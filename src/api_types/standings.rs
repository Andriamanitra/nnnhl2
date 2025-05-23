use crate::api_types::*;

type GoalCount = i32;

/// Number in range 1..=32 that can be used to order the teams
type Rank = u8;

/// Response type for the following end points:
/// * `/v1/standings/now`
/// * `/v1/standings/YYYY-MM-DD`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Standings {
    #[serde(rename = "wildCardIndicator")]
    pub wild_card_indicator: bool,
    #[serde(rename = "standingsDateTimeUtc")]
    pub standings_date_time_utc: String,
    pub standings: Vec<TeamStanding>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamStanding {
    #[serde(rename = "conferenceAbbrev")]
    pub conference_abbrev: String,
    #[serde(rename = "conferenceHomeSequence")]
    pub conference_home_sequence: Rank,
    #[serde(rename = "conferenceL10Sequence")]
    pub conference_l10sequence: Rank,
    #[serde(rename = "conferenceName")]
    pub conference_name: String,
    #[serde(rename = "conferenceRoadSequence")]
    pub conference_road_sequence: Rank,
    #[serde(rename = "conferenceSequence")]
    pub conference_sequence: Rank,
    pub date: String,
    #[serde(rename = "divisionAbbrev")]
    pub division_abbrev: String,
    #[serde(rename = "divisionHomeSequence")]
    pub division_home_sequence: Rank,
    #[serde(rename = "divisionL10Sequence")]
    pub division_l10sequence: Rank,
    #[serde(rename = "divisionName")]
    pub division_name: String,
    #[serde(rename = "divisionRoadSequence")]
    pub division_road_sequence: Rank,
    #[serde(rename = "divisionSequence")]
    pub division_sequence: Rank,
    #[serde(rename = "gameTypeId")]
    pub game_type_id: GameTypeId,
    #[serde(rename = "gamesPlayed")]
    pub games_played: usize,
    #[serde(rename = "goalDifferential")]
    pub goal_differential: GoalCount,
    #[serde(rename = "goalDifferentialPctg")]
    pub goal_differential_pctg: f64,
    #[serde(rename = "goalAgainst")]
    pub goal_against: GoalCount,
    #[serde(rename = "goalFor")]
    pub goal_for: GoalCount,
    #[serde(rename = "goalsForPctg")]
    pub goals_for_pctg: f64,
    #[serde(rename = "homeGamesPlayed")]
    pub home_games_played: usize,
    #[serde(rename = "homeGoalDifferential")]
    pub home_goal_differential: GoalCount,
    #[serde(rename = "homeGoalsAgainst")]
    pub home_goals_against: GoalCount,
    #[serde(rename = "homeGoalsFor")]
    pub home_goals_for: GoalCount,
    #[serde(rename = "homeLosses")]
    pub home_losses: usize,
    #[serde(rename = "homeOtLosses")]
    pub home_ot_losses: usize,
    #[serde(rename = "homePoints")]
    pub home_points: usize,
    #[serde(rename = "homeRegulationPlusOtWins")]
    pub home_regulation_plus_ot_wins: usize,
    #[serde(rename = "homeRegulationWins")]
    pub home_regulation_wins: usize,
    #[serde(rename = "homeTies")]
    pub home_ties: usize,
    #[serde(rename = "homeWins")]
    pub home_wins: usize,
    #[serde(rename = "l10GamesPlayed")]
    pub l10games_played: usize,
    #[serde(rename = "l10GoalDifferential")]
    pub l10goal_differential: GoalCount,
    #[serde(rename = "l10GoalsAgainst")]
    pub l10goals_against: GoalCount,
    #[serde(rename = "l10GoalsFor")]
    pub l10goals_for: GoalCount,
    #[serde(rename = "l10Losses")]
    pub l10losses: usize,
    #[serde(rename = "l10OtLosses")]
    pub l10ot_losses: usize,
    #[serde(rename = "l10Points")]
    pub l10points: usize,
    #[serde(rename = "l10RegulationPlusOtWins")]
    pub l10regulation_plus_ot_wins: usize,
    #[serde(rename = "l10RegulationWins")]
    pub l10regulation_wins: usize,
    #[serde(rename = "l10Ties")]
    pub l10ties: usize,
    #[serde(rename = "l10Wins")]
    pub l10wins: usize,
    #[serde(rename = "leagueHomeSequence")]
    pub league_home_sequence: Rank,
    #[serde(rename = "leagueL10Sequence")]
    pub league_l10sequence: Rank,
    #[serde(rename = "leagueRoadSequence")]
    pub league_road_sequence: Rank,
    #[serde(rename = "leagueSequence")]
    pub league_sequence: Rank,
    pub losses: usize,
    #[serde(rename = "otLosses")]
    pub ot_losses: usize,
    #[serde(rename = "placeName")]
    pub place_name: LocalizableString,
    #[serde(rename = "pointPctg")]
    pub point_pctg: f64,
    pub points: usize,
    #[serde(rename = "regulationPlusOtWinPctg")]
    pub regulation_plus_ot_win_pctg: f64,
    #[serde(rename = "regulationPlusOtWins")]
    pub regulation_plus_ot_wins: usize,
    #[serde(rename = "regulationWinPctg")]
    pub regulation_win_pctg: f64,
    #[serde(rename = "regulationWins")]
    pub regulation_wins: usize,
    #[serde(rename = "roadGamesPlayed")]
    pub road_games_played: usize,
    #[serde(rename = "roadGoalDifferential")]
    pub road_goal_differential: GoalCount,
    #[serde(rename = "roadGoalsAgainst")]
    pub road_goals_against: GoalCount,
    #[serde(rename = "roadGoalsFor")]
    pub road_goals_for: GoalCount,
    #[serde(rename = "roadLosses")]
    pub road_losses: usize,
    #[serde(rename = "roadOtLosses")]
    pub road_ot_losses: usize,
    #[serde(rename = "roadPoints")]
    pub road_points: usize,
    #[serde(rename = "roadRegulationPlusOtWins")]
    pub road_regulation_plus_ot_wins: usize,
    #[serde(rename = "roadRegulationWins")]
    pub road_regulation_wins: usize,
    #[serde(rename = "roadTies")]
    pub road_ties: usize,
    #[serde(rename = "roadWins")]
    pub road_wins: usize,
    #[serde(rename = "seasonId")]
    pub season_id: SeasonId,
    #[serde(rename = "shootoutLosses")]
    pub shootout_losses: usize,
    #[serde(rename = "shootoutWins")]
    pub shootout_wins: usize,
    #[serde(rename = "streakCode")]
    pub streak_code: String,
    #[serde(rename = "streakCount")]
    pub streak_count: usize,
    #[serde(rename = "teamName")]
    pub team_name: LocalizableString,
    #[serde(rename = "teamCommonName")]
    pub team_common_name: LocalizableString,
    #[serde(rename = "teamAbbrev")]
    pub team_abbrev: LocalizableString,
    #[serde(rename = "teamLogo")]
    pub team_logo: String,
    pub ties: usize,
    #[serde(rename = "waiversSequence")]
    pub waivers_sequence: Rank,
    #[serde(rename = "wildcardSequence")]
    pub wildcard_sequence: Rank,
    #[serde(rename = "winPctg")]
    pub win_pctg: f64,
    pub wins: usize,
}

/// Response type for the following end points:
/// * `/v1/standings-season`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandingsSeasons {
    #[serde(rename = "currentDate")]
    pub current_date: String,
    pub seasons: Vec<Season>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: SeasonId,
    #[serde(rename = "conferencesInUse")]
    pub conferences_in_use: bool,
    #[serde(rename = "divisionsInUse")]
    pub divisions_in_use: bool,
    #[serde(rename = "pointForOTlossInUse")]
    pub point_for_otloss_in_use: bool,
    #[serde(rename = "regulationWinsInUse")]
    pub regulation_wins_in_use: bool,
    #[serde(rename = "rowInUse")]
    pub row_in_use: bool,
    #[serde(rename = "standingsEnd")]
    pub standings_end: String,
    #[serde(rename = "standingsStart")]
    pub standings_start: String,
    #[serde(rename = "tiesInUse")]
    pub ties_in_use: bool,
    #[serde(rename = "wildcardInUse")]
    pub wildcard_in_use: bool,
}
