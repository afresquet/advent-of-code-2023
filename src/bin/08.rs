use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Node<'a>(&'a str, &'a str);

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = alpha1(input)?;
    let instructions = instructions.chars().map(Into::into).collect();
    Ok((input, instructions))
}

fn parse_node(input: &str) -> IResult<&str, (&str, Node<'_>)> {
    let (input, (node, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, (node, Node(left, right))))
}

fn parse_nodes(input: &str) -> IResult<&str, BTreeMap<&str, Node<'_>>> {
    let (input, nodes) = separated_list1(newline, parse_node)(input)?;
    Ok((input, BTreeMap::from_iter(nodes)))
}

fn parse_document(input: &str) -> IResult<&str, (Vec<Instruction>, BTreeMap<&str, Node<'_>>)> {
    let (input, instructions) = terminated(parse_instructions, multispace1)(input)?;
    let (input, nodes) = parse_nodes(input)?;
    Ok((input, (instructions, nodes)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (instructions, nodes)) = parse_document(input).unwrap();
    let mut node = "AAA";
    let mut steps = 0;
    for (step, instruction) in instructions.iter().cycle().enumerate() {
        if node == "ZZZ" {
            steps = step;
            break;
        }
        let Node(left, right) = nodes.get(node).unwrap();
        node = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        }
    }
    Some(steps as u32)
}

fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, 0) => 0,
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (instructions, nodes)) = parse_document(input).unwrap();
    let starting_nodes = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    let steps_for_nodes = starting_nodes.into_iter().map(|node| {
        let mut node = *node;
        let mut steps = 0;
        for (step, instruction) in instructions.iter().cycle().enumerate() {
            if node.ends_with('Z') {
                steps = step;
                break;
            }
            let Node(left, right) = nodes.get(node).unwrap();
            node = match instruction {
                Instruction::Left => left,
                Instruction::Right => right,
            }
        }
        println!("{steps}");
        steps as u64
    });

    steps_for_nodes.reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_extra() {
        let result = part_one(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
