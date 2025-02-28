use crate::api_types::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub season: SeasonId,
    #[serde(rename = "gameType")]
    pub game_type: GameTypeId,
    pub venue: LocalizableString,
    #[serde(rename = "neutralSite")]
    pub neutral_site: bool,
    #[serde(rename = "startTimeUTC")]
    pub start_time_utc: DateTime<Utc>,
    #[serde(rename = "easternUTCOffset")]
    pub eastern_utcoffset: String,
    #[serde(rename = "venueUTCOffset")]
    pub venue_utcoffset: String,
    #[serde(rename = "venueTimezone")]
    pub venue_timezone: String,
    #[serde(rename = "gameState")]
    /// Known values for game_state are
    /// * `"OFF"` - game has ended
    /// * ??????? - game is ongoing
    /// * `"FUT"` - game has not started yet
    pub game_state: String,
    #[serde(rename = "gameScheduleState")]
    pub game_schedule_state: String,
    #[serde(rename = "tvBroadcasts")]
    pub tv_broadcasts: Vec<TvBroadcast>,
    #[serde(rename = "awayTeam")]
    pub away_team: AwayTeam,
    #[serde(rename = "homeTeam")]
    pub home_team: HomeTeam,
    #[serde(rename = "periodDescriptor")]
    pub period_descriptor: PeriodDescriptor,
    #[serde(rename = "gameOutcome")]
    pub game_outcome: Option<GameOutcome>,
    #[serde(rename = "winningGoalie")]
    pub winning_goalie: Option<WinningGoalie>,
    #[serde(rename = "winningGoalScorer")]
    pub winning_goal_scorer: Option<WinningGoalie>,
    #[serde(rename = "threeMinRecap")]
    pub three_min_recap: Option<String>,
    #[serde(rename = "threeMinRecapFr")]
    pub three_min_recap_fr: Option<String>,
    #[serde(rename = "condensedGame")]
    pub condensed_game: Option<String>,
    #[serde(rename = "condensedGameFr")]
    pub condensed_game_fr: Option<String>,
    #[serde(rename = "gameCenterLink")]
    pub game_center_link: String,
    #[serde(rename = "ticketsLink")]
    pub tickets_link: Option<String>,
    #[serde(rename = "ticketsLinkFr")]
    pub tickets_link_fr: Option<String>,
    #[serde(rename = "specialEvent")]
    pub special_event: Option<SpecialEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TvBroadcast {
    pub id: i64,
    pub market: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub network: String,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwayTeam {
    pub id: TeamId,
    #[serde(rename = "commonName")]
    pub common_name: LocalizableString,
    #[serde(rename = "placeName")]
    pub place_name: LocalizableString,
    #[serde(rename = "placeNameWithPreposition")]
    pub place_name_with_preposition: LocalizableString,
    pub abbrev: String,
    pub logo: String,
    #[serde(rename = "darkLogo")]
    pub dark_logo: String,
    #[serde(rename = "awaySplitSquad")]
    pub away_split_squad: bool,
    pub score: Option<i64>,
    #[serde(rename = "radioLink")]
    pub radio_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeTeam {
    pub id: TeamId,
    #[serde(rename = "commonName")]
    pub common_name: LocalizableString,
    #[serde(rename = "placeName")]
    pub place_name: LocalizableString,
    #[serde(rename = "placeNameWithPreposition")]
    pub place_name_with_preposition: LocalizableString,
    pub abbrev: String,
    pub logo: String,
    #[serde(rename = "darkLogo")]
    pub dark_logo: String,
    #[serde(rename = "homeSplitSquad")]
    pub home_split_squad: bool,
    pub score: Option<i64>,
    #[serde(rename = "radioLink")]
    pub radio_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodDescriptor {
    pub number: i64,
    #[serde(rename = "periodType")]
    pub period_type: String,
    #[serde(rename = "maxRegulationPeriods")]
    pub max_regulation_periods: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameOutcome {
    #[serde(rename = "lastPeriodType")]
    pub last_period_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WinningGoalie {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "firstInitial")]
    pub first_initial: LocalizableString,
    #[serde(rename = "lastName")]
    pub last_name: LocalizableString,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialEvent {
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    pub name: LocalizableString,
    #[serde(rename = "lightLogoUrl")]
    pub light_logo_url: LocalizableString,
}
