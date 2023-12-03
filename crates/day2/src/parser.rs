use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0, space1};
use nom::multi::separated_list1;
use nom::*;
use serde::{Deserialize, Serialize};

/// Root data structure, contains the id of the game and the sets that were played
#[derive(Serialize, Debug, Clone)]
pub(crate) struct GameRun {
    pub(crate) id: i32,
    pub(crate) sets: GameSets,
}

/// Collection of [PlayedSet]
#[derive(Serialize, Debug, Clone)]
pub(crate) struct GameSets(Vec<PlayedSet>);

/// A collection of [Color]s with their quantity
#[derive(Serialize, Debug, Clone)]
pub(crate) struct PlayedSet(Vec<Color>);

/// Red, Green or Blue colors
#[derive(Serialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    /// Red color
    Red(i32),
    /// Green Color
    Green(i32),
    /// Blue Color
    Blue(i32),
}

/// Root parser that parses a [GameRun] from a [string slice][std::str],
/// coming from a [line][std::io::Lines] of the file
pub fn game_run(input: &str) -> IResult<&str, GameRun> {
    let (input, game_id) = game_id(input)?;
    let (input, _) = space1(input)?;
    let (input, game_sets) = game_sets(input)?;
    Ok((
        input,
        GameRun {
            id: game_id,
            sets: GameSets(game_sets),
        },
    ))
}

/// Parses the id (number) from each "Game" entry
pub fn game_id(input: &str) -> IResult<&str, i32> {
    let (input, _game_tag) = tag("Game")(input)?;
    let (input, _) = space1(input)?;
    let (input, game_id) = character::complete::i32(input)?;
    let (input, _) = char(':')(input)?;
    Ok((input, game_id))
}

/// Parses a [Vec][std::vec] of [PlayedSet]
pub fn game_sets(input: &str) -> IResult<&str, Vec<PlayedSet>> {
    separated_list1(char(';'), game_set)(input)
}

/// Parses a [PlayedSet], a list of [Color]s with count separated by `,` and delimited by `;`
pub fn game_set(input: &str) -> IResult<&str, PlayedSet> {
    let (input, vec) = separated_list1(char(','), set_entry)(input)?;
    Ok((input, PlayedSet(vec)))
}

/// Parses a [Color] with its count
pub fn set_entry(input: &str) -> IResult<&str, Color> {
    let (input, _) = space0(input)?;
    let (input, count) = character::complete::i32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = color(input)?;
    let color = match color {
        Color::Red(_) => Color::Red(count),
        Color::Green(_) => Color::Green(count),
        Color::Blue(_) => Color::Blue(count),
    };
    Ok((input, color))
}

/// Parse one of [Color::Red], [Color::Green] or [Color::Blue]
pub fn color(input: &str) -> IResult<&str, Color> {
    alt((red, green, blue))(input)
}

/// Parses the string "green" into a [Color::Red]
pub fn red(input: &str) -> IResult<&str, Color> {
    let (output, _) = tag("red")(input)?;
    Ok((output, Color::Red(0)))
}

/// Parses the string "green" into a [Color::Green]
pub fn green(input: &str) -> IResult<&str, Color> {
    let (output, _) = tag("green")(input)?;
    Ok((output, Color::Green(0)))
}

/// Parses the string "blue" into a [Color::Blue]
pub fn blue(input: &str) -> IResult<&str, Color> {
    let (output, _) = tag("blue")(input)?;
    Ok((output, Color::Blue(0)))
}
