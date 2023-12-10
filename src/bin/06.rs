use std::{num::ParseIntError, str::FromStr};

use nom::{
    character::complete::{self, alpha1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn record_beating_count(&self) -> u64 {
        let mut range = 1..self.time;

        let find_record = |held_time: &u64| {
            let remaining_time = self.time - held_time;
            let distance_traveled = held_time * remaining_time;
            distance_traveled > self.distance
        };

        let pad_start = range.find(find_record).unwrap();
        let pad_end = range.rev().find(find_record).unwrap();

        self.time - (pad_start - 1) - (self.time - pad_end)
    }
}

fn parse_segment(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, values) = preceded(
        tuple((alpha1, complete::char(':'), space1)),
        separated_list1(space1, complete::u64),
    )(input)?;

    Ok((input, values))
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (times, distances)) = separated_pair(parse_segment, newline, parse_segment)(input)?;

    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    Ok((input, races))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, races) = parse_races(input).unwrap();

    let record_beating_races = races.iter().map(|race| race.record_beating_count());

    Some(record_beating_races.product())
}

fn merge_values<T: FromStr<Err = ParseIntError> + ToString>(
    values: &[T],
) -> Result<T, ParseIntError> {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<String>()
        .parse::<T>()
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (input, (times, distances)) = separated_pair(parse_segment, newline, parse_segment)(input)?;
    let time = merge_values(&times).unwrap();
    let distance = merge_values(&distances).unwrap();
    Ok((input, Race { time, distance }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, race) = parse_race(input).unwrap();

    Some(race.record_beating_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
