use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Point {
    fn go(&self, direction: &Direction) -> Point {
        let (dx, dy) = direction.vector();
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }

    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[aoc_generator(day3)]
fn preprocess(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect()
}

#[aoc(day3, part1)]
fn part1(directions: &[Direction]) -> usize {
    let mut point = Point::default();
    let mut houses = 1;
    let mut visited = HashSet::new();
    visited.insert(point);

    for direction in directions {
        point = point.go(direction);
        if visited.contains(&point) {
            continue;
        } else {
            visited.insert(point);
            houses += 1;
        }
    }

    houses
}

#[aoc(day3, part2)]
fn part2(directions: &[Direction]) -> usize {
    let mut santa = Point::default();
    let mut robot = Point::default();
    let mut visited = HashSet::new();
    let mut houses = 1;
    visited.insert(santa);

    let mut move_and_visit = |character: &mut Point, direction: &Direction| {
        *character = character.go(direction);
        if !visited.contains(character) {
            visited.insert(*character);
            houses += 1;
        }
    };
    let mut turn = true;
    for direction in directions.iter() {
        if turn {
            move_and_visit(&mut santa, direction);
        } else {
            move_and_visit(&mut robot, direction);
        }
        turn = !turn;
    }

    houses
}
