use std::fmt::Display;
// Copied from https://github.com/PoiScript/iso8601-duration
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res, opt},
    error::{ErrorKind, ParseError},
    number::complete::float,
    sequence::{preceded, separated_pair, terminated, tuple},
    Err, IResult,
};

#[derive(Debug, PartialEq)]
pub struct Duration {
    pub year: f32,
    pub month: f32,
    pub day: f32,
    pub hour: f32,
    pub minute: f32,
    pub second: f32,
}

impl Duration {
    pub fn parse(input: &str) -> Result<Duration, Err<(&str, ErrorKind)>> {
        let (_, duration) = all_consuming(preceded(
            tag("P"),
            alt((parse_week_format, parse_basic_format)),
        ))(input)
        .unwrap();
        Ok(duration)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hour > 0.0 {
            write!(f, "{} hours and {} minutes", self.hour, self.minute)
        } else {
            write!(f, "{} minutes", self.minute)
        }
    }
}

fn decimal_comma_number(input: &str) -> IResult<&str, f32> {
    map_res(separated_pair(digit1, tag(","), digit1), |(a, b)| {
        f32::from_str(&format!("{a}.{b}"))
    })(input)
}

fn value_with_designator(designator: &str) -> impl Fn(&str) -> IResult<&str, f32> + '_ {
    move |input| {
        terminated(
            alt((float, decimal_comma_number, map_res(digit1, f32::from_str))),
            tag(designator),
        )(input)
    }
}

fn parse_basic_format(input: &str) -> IResult<&str, Duration> {
    let (input, (year, month, day)) = tuple((
        opt(value_with_designator("Y")),
        opt(value_with_designator("M")),
        opt(value_with_designator("D")),
    ))(input)?;

    let (input, time) = opt(preceded(
        tag("T"),
        tuple((
            opt(value_with_designator("H")),
            opt(value_with_designator("M")),
            opt(value_with_designator("S")),
        )),
    ))(input)?;

    let (hour, minute, second) = time.unwrap_or_default();

    if year.is_none()
        && month.is_none()
        && day.is_none()
        && hour.is_none()
        && minute.is_none()
        && second.is_none()
    {
        Err(Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Verify,
        )))
    } else {
        Ok((
            input,
            Duration {
                year: year.unwrap_or_default(),
                month: month.unwrap_or_default(),
                day: day.unwrap_or_default(),
                hour: hour.unwrap_or_default(),
                minute: minute.unwrap_or_default(),
                second: second.unwrap_or_default(),
            },
        ))
    }
}

fn parse_week_format(input: &str) -> IResult<&str, Duration> {
    let (input, week) = value_with_designator("W")(input)?;

    Ok((
        input,
        Duration {
            year: 0.,
            month: 0.,
            day: week * 7.,
            hour: 0.,
            minute: 0.,
            second: 0.,
        },
    ))
}

fn _parse_extended_format(_input: &str) -> IResult<&str, Duration> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Duration;

    #[test]
    pub fn simple() {
        assert_eq!(
            Duration::parse("PT50M").unwrap(),
            Duration {
                year: 0.0,
                month: 0.0,
                day: 0.0,
                hour: 0.0,
                minute: 50.0,
                second: 0.0
            }
        );
        assert_eq!(
            Duration::parse("P0DT1H45M").unwrap(),
            Duration {
                year: 0.0,
                month: 0.0,
                day: 0.0,
                hour: 1.0,
                minute: 45.0,
                second: 0.0
            }
        );
    }
}
