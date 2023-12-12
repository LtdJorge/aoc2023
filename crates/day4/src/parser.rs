use crate::data::Card;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32 as nom_i32, space0, space1};
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;

pub fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = header(input)?;
    let (input, num_list) = separated_list1(char('|'), num_list)(input)?;
    if num_list.len() == 2 {
        Ok((
            input,
            Card {
                id,
                winning: num_list[0].clone(),
                owned: num_list[1].clone(),
            },
        ))
    } else {
        Err(nom::Err::Failure(nom::error::Error {
            input,
            code: nom::error::ErrorKind::SeparatedList,
        }))
    }
}

pub fn header(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space0(input)?;
    let (input, index) = nom_i32(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space0(input)?;
    Ok((input, index))
}

pub fn num_list(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = space0(input)?;
    let (input, list) = separated_list0(space1, nom_i32)(input)?;
    let (input, _) = space0(input)?;
    Ok((input, list))
}
