use std::collections::BTreeMap;

use nom::{
    character::complete::{self, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eigth = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    pub fn strength(&self, joker: bool) -> u32 {
        match self {
            Self::J if joker => 1,
            card => (*card) as u32,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eigth,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn new(cards: &Vec<Card>, joker: bool) -> Self {
        let mut amounts: BTreeMap<&Card, u32> = BTreeMap::new();
        for card in cards {
            amounts
                .entry(card)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        }
        if joker && amounts.len() != 1 {
            if let Some(jokers) = amounts.remove(&Card::J) {
                let mut values = Vec::from_iter(amounts.iter());
                values.sort_by(|(_, a), (_, b)| b.cmp(a));
                let biggest = values.first().unwrap().0;
                amounts
                    .entry(biggest)
                    .and_modify(|amount| *amount += jokers);
            }
        }
        match amounts.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if amounts.values().any(|amount| *amount == 4) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if amounts.values().any(|amount| *amount == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
    joker: bool,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32, joker: bool) -> Self {
        let hand_type = HandType::new(&cards, joker);

        Self {
            cards,
            bid,
            hand_type,
            joker,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(c1, c2)| {
                        match c1.strength(self.joker).cmp(&c2.strength(self.joker)) {
                            std::cmp::Ordering::Equal => None,
                            ordering => Some(ordering),
                        }
                    })
            }
            ordering => Some(ordering),
        }
    }
}

fn parse_hand(joker: bool) -> impl Fn(&str) -> IResult<&str, Hand> {
    move |input: &str| -> IResult<&str, Hand> {
        let (input, (cards, bid)) = separated_pair(alphanumeric1, space1, complete::u32)(input)?;
        let cards = cards.chars().map(Into::into).collect();
        Ok((input, Hand::new(cards, bid, joker)))
    }
}

fn parse_hands(input: &str, joker: bool) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, parse_hand(joker))(input)
}

fn solve_puzzle(input: &str, joker: bool) -> Option<u32> {
    let (_, mut hands) = parse_hands(input, joker).unwrap();

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1));

    Some(winnings.sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_puzzle(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_puzzle(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
