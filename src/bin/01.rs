advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars
                .find(|char| char.is_ascii_digit())
                .expect("No numbers in line");
            let second = chars
                .rev()
                .find(|char| char.is_ascii_digit())
                .unwrap_or(first);
            format!("{}{}", first, second).parse::<u32>().unwrap()
        })
        .sum();

    Some(result)
}

const NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

struct Number<'a>(&'a str);

impl<'a> From<&Number<'a>> for u32 {
    fn from(value: &Number) -> Self {
        match value.0 {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => unreachable!(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let values = lines.map(|line| {
        let mut index = 0;
        let mut numbers = vec![];
        while !line[index..].is_empty() {
            for num in NUMBERS {
                if line[index..].starts_with(num) {
                    let number = &line[index..(index + num.len())];
                    numbers.push(Number(number));
                    index += num.len() - 1;
                    break;
                }
            }

            index += 1;
        }

        let mut iter = numbers.iter();
        let first: u32 = iter.next().expect("No numbers in line").into();
        let second: u32 = iter.last().map_or(first, |number| number.into());

        first * 10 + second
    });
    Some(values.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, Some(281));
    }
}
