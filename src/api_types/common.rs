use crate::api_types::*;

/// Unique numeric identifier for the season.
/// `SeasonId`s are made by concatenating the digits of the years during which
/// the season takes place (eg. `20242025` for the 2024-2025 season).
pub type SeasonId = u32;

/// Unique numeric identifier for a game.
///
/// Example: 2024020797
/// * The first 4 digits tell you which season the game was played (the year the season started).
/// * The next two digits tell you the type of the game: 01 = preseason, 02 = regular season, 03 = playoffs, 04 = all-star
/// * The last 4 digits are the game number
pub type GameId = u32;

/// Unique numeric identifier for a team.
pub type TeamId = u32;

/// Numeric identifier that represents the game type.
///
/// Known values are 1 = preseason, 2 = regular season, 3 = playoffs, 4 = all-star game
pub type GameTypeId = i32;

/// Represents a string that may have different representations in different languages.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    default: String,
    fr: Option<String>,
    sk: Option<String>,
    fi: Option<String>,
    sv: Option<String>,
    cs: Option<String>,
    de: Option<String>,
    es: Option<String>,
}
