use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: BTreeSet<u32>,
    owned_numbers: BTreeSet<u32>,
}

impl Card {
    pub fn calculate_points(&self) -> u32 {
        let mut points = 0;

        for _ in 0..self.calculate_matching_numbers() {
            if points == 0 {
                points = 1
            } else {
                points *= 2
            }
        }

        points
    }

    pub fn calculate_matching_numbers(&self) -> u32 {
        self.owned_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(*number))
            .count() as u32
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = pair(tag("Card"), space1)(input)?;
    let (input, id) = complete::u32(input)?;
    let (input, _) = pair(tag(":"), space1)(input)?;
    let (input, (winning_numbers, owned_numbers)) = separated_pair(
        parse_numbers,
        delimited(space1, tag("|"), space1),
        parse_numbers,
    )(input)?;

    Ok((
        input,
        Card {
            id,
            winning_numbers: winning_numbers.into_iter().collect(),
            owned_numbers: owned_numbers.into_iter().collect(),
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, parse_card)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = parse_cards(input).unwrap();

    Some(cards.iter().map(|card| card.calculate_points()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cards) = parse_cards(input).unwrap();
    let mut copies: BTreeMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let copies_won = card.calculate_matching_numbers();
        let copies_of_current = *copies.get(&card.id).unwrap();
        for i in (card.id + 1)..=(card.id + copies_won) {
            copies
                .entry(i)
                .and_modify(|value| *value += copies_of_current);
        }
    }

    Some(copies.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
