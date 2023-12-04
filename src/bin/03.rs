use nom::{bytes::complete::take_till, character::complete::digit0, IResult};

advent_of_code::solution!(3);

#[derive(Debug)]
struct PartNumber<'a> {
    number: &'a str,
    position: (usize, usize),
}

impl PartNumber<'_> {
    pub fn is_valid(&self, grid: &[Vec<char>]) -> bool {
        let row_indeces =
            (self.position.0.checked_sub(1).unwrap_or(self.position.0))..=(self.position.0 + 1);
        let column_indeces = (self.position.1.checked_sub(1).unwrap_or(self.position.1))
            ..=(self.position.1 + self.number.len());

        for row in row_indeces {
            for column in column_indeces.clone() {
                if let Some(symbol) = grid.get(row).and_then(|row| row.get(column)) {
                    if !symbol.is_ascii_digit() && *symbol != '.' {
                        return true;
                    }
                }
            }
        }

        false
    }
}

fn parse_line_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    let mut string = input;
    let mut numbers = vec![];
    while !string.is_empty() {
        let (input, _) = take_till(|c: char| c.is_ascii_digit())(string)?;
        let (input, number) = digit0(input)?;
        if !number.is_empty() {
            numbers.push(number);
        }
        string = input;
    }

    Ok((string, numbers))
}

fn parse_numbers(input: &str) -> Vec<PartNumber> {
    let numbers = input.lines().enumerate().flat_map(|(row, line)| {
        let (_, numbers) = parse_line_numbers(line).unwrap();

        numbers
            .iter()
            .map(|number| {
                let column = line.find(number).unwrap();
                PartNumber {
                    number,
                    position: (row, column),
                }
            })
            .collect::<Vec<_>>()
    });

    numbers.collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let numbers = parse_numbers(input);

    let part_numbers = numbers.iter().filter_map(|number| {
        number
            .is_valid(&grid)
            .then_some(number.number.parse::<u32>().unwrap())
    });

    Some(part_numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_number_side() {
        let result = part_one(
            ".....
.123.
.._..",
        );
        assert_eq!(result, Some(123));
    }

    #[test]
    fn part_number_diagonal() {
        let result = part_one(
            ".....
.123.
...._",
        );
        assert_eq!(result, Some(123));
    }

    #[test]
    fn part_number_small_grid() {
        let result = part_one("123_");
        assert_eq!(result, Some(123));
    }

    #[test]
    fn no_part_number() {
        let result = part_one("123");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn single_part_number() {
        let result = part_one(
            ".........
.123.321.
..._.....",
        );
        assert_eq!(result, Some(123));
    }

    #[test]
    fn double_part_number() {
        let result = part_one(
            ".........
.123.321.
...._....",
        );
        assert_eq!(result, Some(444));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(540131));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
