use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize{
    let mut floor = 0;
    for (position, movement) in input.chars().enumerate() {
        match movement {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return position + 1;
        }
    }
    0
}