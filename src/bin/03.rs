use std::{collections::BTreeMap, ops::RangeInclusive};

use nom::{bytes::complete::take_till, character::complete::digit0, IResult};

advent_of_code::solution!(3);

#[derive(Debug)]
struct PartNumber<'a> {
    number: &'a str,
    position: (usize, usize),
}

impl PartNumber<'_> {
    fn ranges(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let start_row = self.position.0.checked_sub(1).unwrap_or(self.position.0);
        let end_row = self.position.0 + 1;
        let start_column = self.position.1.checked_sub(1).unwrap_or(self.position.1);
        let end_column = self.position.1 + self.number.len();

        (start_row..=end_row, start_column..=end_column)
    }

    pub fn is_valid(&self, grid: &[Vec<char>]) -> bool {
        let (row_range, column_range) = self.ranges();

        for row in row_range {
            for column in column_range.clone() {
                if let Some(symbol) = grid.get(row).and_then(|row| row.get(column)) {
                    if !symbol.is_ascii_digit() && *symbol != '.' {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_touching_gear(&self, grid: &[Vec<char>]) -> Option<(usize, usize)> {
        let (row_range, column_range) = self.ranges();

        for row in row_range {
            for column in column_range.clone() {
                if let Some(symbol) = grid.get(row).and_then(|row| row.get(column)) {
                    if *symbol == '*' {
                        return Some((row, column));
                    }
                }
            }
        }

        None
    }
}

impl<'a> From<(&'a str, usize, usize)> for PartNumber<'a> {
    fn from((number, row, column): (&'a str, usize, usize)) -> Self {
        Self {
            number,
            position: (row, column),
        }
    }
}

fn parse_line_numbers(input: &str, row: usize) -> IResult<&str, Vec<(&str, usize, usize)>> {
    let mut numbers = vec![];
    let mut remaining = input;
    let original_length = input.len();
    while !remaining.is_empty() {
        let (s, _) = take_till(|c: char| c.is_ascii_digit())(remaining)?;
        let (s, n) = digit0(s)?;
        remaining = s;
        if !n.is_empty() {
            numbers.push((n, row, original_length - remaining.len() - n.len()));
        }
    }

    Ok((remaining, numbers))
}

fn parse_numbers(input: &str) -> Vec<PartNumber> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let (_, numbers) = parse_line_numbers(line, row).unwrap();

            numbers
        })
        .map(Into::into)
        .collect()
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let numbers = parse_numbers(input);

    let part_numbers = numbers.iter().filter_map(|number| {
        number
            .is_valid(&grid)
            .then_some(number.number.parse::<u32>().unwrap())
    });

    Some(part_numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let numbers = parse_numbers(input);
    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();

    for number in numbers {
        if let Some(coordinates) = number.is_touching_gear(&grid) {
            let n = number.number.parse::<u32>().unwrap();
            gears
                .entry(coordinates)
                .and_modify(|v| v.push(n))
                .or_insert_with(|| vec![n]);
        }
    }

    let gear_ratios = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>());

    Some(gear_ratios.sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
