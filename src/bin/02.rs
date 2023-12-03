use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(2);

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    pub fn new(color: &str, amount: u32) -> Self {
        match color {
            "red" => Self::Red(amount),
            "green" => Self::Green(amount),
            "blue" => Self::Blue(amount),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    pub fn is_valid(&self) -> bool {
        let Self { red, green, blue } = *self;
        red <= 12 && green <= 13 && blue <= 14
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.cube_sets.iter().all(|cube_set| cube_set.is_valid())
    }

    pub fn power_of_minimum_blocks(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cube_set in self.cube_sets.iter() {
            red = red.max(cube_set.red);
            green = green.max(cube_set.green);
            blue = blue.max(cube_set.blue);
        }
        red * green * blue
    }
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, space1, alpha1)(input)?;
    let cube = Cube::new(color, amount);
    Ok((input, cube))
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let (input, cubes) = separated_list0(tag(", "), parse_cube)(input)?;
    for cube in cubes {
        match cube {
            Cube::Red(amount) => red += amount,
            Cube::Green(amount) => green += amount,
            Cube::Blue(amount) => blue += amount,
        }
    }
    Ok((input, CubeSet { red, green, blue }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), complete::u32, tag(": "))(input)?;
    let (input, cube_sets) = separated_list1(tag("; "), parse_cube_set)(input)?;
    Ok((input, Game { id, cube_sets }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_games(input).unwrap();

    let valid_game_ids = games
        .iter()
        .filter_map(|game| game.is_valid().then_some(game.id));

    Some(valid_game_ids.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = parse_games(input).unwrap();

    let game_set_values = games.iter().map(|game| game.power_of_minimum_blocks());

    Some(game_set_values.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
