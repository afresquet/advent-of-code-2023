use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Seed(u64);

impl Seed {
    pub fn get_location(&self, maps: &Map) -> u64 {
        let mut location = self.0;
        for ranges in maps {
            if let Some(range) = ranges.iter().find(|range| {
                (range.source_range_start..(range.source_range_start + range.range_length))
                    .contains(&location)
            }) {
                location = location - range.source_range_start + range.destination_range_start;
            }
        }
        location
    }
}

#[derive(Debug)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

type Map = Vec<Vec<Range>>;

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, destination_range_start) = complete::u64(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = complete::u64(input)?;
    let (input, _) = space1(input)?;
    let (input, range_length) = complete::u64(input)?;

    Ok((
        input,
        Range {
            destination_range_start,
            source_range_start,
            range_length,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, (_from, _to)) =
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"))(input)?;
    let (input, ranges) = separated_list1(line_ending, parse_range)(input)?;

    Ok((input, ranges))
}

fn parse_seeds_and_maps(input: &str) -> IResult<&str, (Vec<u64>, Map)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, complete::u64),
        multispace1,
    )(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), parse_map)(input.trim_end())?;

    Ok((input, (seeds, maps)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (seeds, maps)) = parse_seeds_and_maps(input).unwrap();
    seeds
        .iter()
        .map(|seed| Seed(*seed).get_location(&maps))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (seeds, maps)) = parse_seeds_and_maps(input).unwrap();
    let seeds = seeds.par_iter().chunks(2).flat_map(|chunk| {
        (*chunk[0]..(chunk[0] + chunk[1]))
            .map(Seed)
            .collect::<Vec<_>>()
    });
    seeds.map(|seed: Seed| seed.get_location(&maps)).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
