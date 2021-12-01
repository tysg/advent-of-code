use super::Rule;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::{complete, opt};
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn parse(input: &str) -> Rule {
    let (_, (parent, _, _, children)) =
        complete(tuple((node, space1, tag("contain"), children_nodes)))(input).unwrap();

    (parent, children)
}

fn children_nodes(i: &str) -> IResult<&str, Vec<(String, u32)>> {
    alt((
        preceded(space1, leaf),
        many1(preceded(space1, numbered_node)),
    ))(i)
}

fn numbered_node(i: &str) -> IResult<&str, (String, u32)> {
    let (i, (num, _, bag)) = tuple((digit1, space1, node))(i)?;
    let num = num.parse::<u32>().unwrap();
    Ok((i, (bag, num)))
}

fn node(input: &str) -> IResult<&str, String> {
    terminated(
        color,
        tuple((tag(" bag"), opt(tag("s")), opt(alt((tag(","), tag(".")))))),
    )(input)
}

fn color(input: &str) -> IResult<&str, String> {
    let (input, parsed) = tuple((alpha1, space1, alpha1))(input)?;
    Ok((input, format!("{} {}", parsed.0, parsed.2)))
}

fn leaf(input: &str) -> IResult<&str, Vec<(String, u32)>> {
    let empty = Vec::new();
    let (input, _) = tag("no other bags.")(input)?;
    Ok((input, empty))
}
