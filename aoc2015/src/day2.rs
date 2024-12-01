use aoc_runner_derive::{aoc, aoc_generator};

struct Gift {
    length: u32,
    width: u32,
    height: u32,
}

impl Gift{
    fn area(&self) -> u32 {
        let surface = 2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length;
        let mut sides = vec![self.length, self.width, self.height];
        sides.sort_unstable();
        let slack = sides[0] * sides[1];
        surface + slack
    }

    fn ribbon(&self) -> u32{
        let mut sides = vec![self.length, self.width, self.height];
        sides.sort_unstable();
        let ribbon = 2 * sides[0] + 2 * sides[1];
        let bow = self.length * self.width * self.height;
        ribbon + bow
    }
}

#[aoc_generator(day2)]
fn preprocess(input: &str) -> Vec<Gift> {
    input.lines()
        .map(|line| {
            let mut parts = line.split('x');
            let length = parts.next().unwrap().parse().unwrap();
            let width = parts.next().unwrap().parse().unwrap();
            let height = parts.next().unwrap().parse().unwrap();
            Gift { length, width, height }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(gifts: &[Gift]) -> u32{
    gifts
        .iter()
        .map(|gift| gift.area())
        .sum()
}

#[aoc(day2, part2)]
fn part2(gifts: &[Gift]) -> u32{
    gifts
        .iter()
        .map(|gift| gift.ribbon())
        .sum()
}
