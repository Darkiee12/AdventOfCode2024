use aoc_runner_derive::aoc;
use md5::Digest;
#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut i = 0;
    loop {
        let hash: Digest = md5::compute(format!("{}{}", input, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return i;
        }
        i += 1;
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut i = 0;
    loop {
        let hash: Digest = md5::compute(format!("{}{}", input, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }
        i += 1;
    }
}