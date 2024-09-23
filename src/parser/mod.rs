// #                 else if (text.StartsWith("p:"))

use nom::{
  branch::alt,
  bytes::complete::{tag, take},
  character::complete::{i64, u32},
  number::complete::double,
  sequence::{delimited, terminated},
  IResult,
};

pub mod action;
pub mod game;

fn read_next_length(input: &str) -> IResult<&str, u32> {
  let (input, length) = terminated(u32, tag(":"))(input)?;
  Ok((input, length))
}

fn read_null(input: &str) -> IResult<&str, ()> {
  let (input, _) = tag("N")(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, ()))
}

// TODO: not sure why he have utf8 encoding, might face issues later on?
fn read_string(input: &str) -> IResult<&str, &str> {
  let (input, _) = tag("s:")(input)?;
  let (input, length) = read_next_length(input)?;
  let (input, string) = delimited(tag("\""), take(length), tag("\";"))(input)?;
  Ok((input, string))
}

fn read_string_opt(input: &str) -> IResult<&str, Option<&str>> {
  read_string(input)
    .map(|v| (v.0, Some(v.1)))
    .or_else(|_| read_null(input).map(|v| (v.0, None)))
}

fn read_i64(input: &str) -> IResult<&str, i64> {
  let (input, _) = tag("i:")(input)?;
  let (input, num) = i64(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, num))
}

fn read_f64(input: &str) -> IResult<&str, f64> {
  let (input, _) = tag("d:")(input)?;
  let (input, num) = double(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, num))
}

fn read_i64_opt(input: &str) -> IResult<&str, Option<i64>> {
  read_i64(input)
    .map(|v| (v.0, Some(v.1)))
    .or_else(|_| read_null(input).map(|v| (v.0, None)))
}

fn read_bool(input: &str) -> IResult<&str, bool> {
  let (input, _) = tag("s:1:")(input)?;
  let (input, bool) = alt::<_, _, nom::error::Error<_>, _>((tag(r#""Y""#), tag(r#""y""#)))(input)
    .map(|(input, _)| (input, true))
    .or_else(|_| alt((tag(r#""N""#), tag(r#""n""#)))(input).map(|(i, _)| (i, false)))?;
  let (input, _) = tag(";")(input)?;
  Ok((input, bool))
}
