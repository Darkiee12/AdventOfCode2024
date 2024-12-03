use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn preprocess(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

trait AOC {
    fn is_safe(&self) -> bool;
}

impl AOC for Vec<i32> {
    fn is_safe(&self) -> bool {
        let (mut asc, mut desc, mut safe) = (true, true, true);
        for pair in self.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            if a >= b {
                asc = false;
            }
            if a <= b {
                desc = false;
            }
            if a.abs_diff(b) > 3 {
                safe = false;
            }
        }
        (asc || desc) && safe
    }
}

#[aoc(day2, part1)]
fn part1(lines: &[Vec<i32>]) -> usize {
    lines.iter().filter(|arr| arr.is_safe()).count()
}

#[aoc(day2, part2, brute_force)]
fn part2(lines: &[Vec<i32>]) -> usize {
    let n = lines.len();
    let unsafe_vec: Vec<Vec<i32>> = lines.iter().filter(|arr| !arr.is_safe()).cloned().collect();
    let mut allowed = n - unsafe_vec.len();
    for vec in unsafe_vec {
        for idx in 0..vec.len() {
            let mut temp = vec[..idx].to_vec();
            temp.extend(vec[idx + 1..].to_vec());
            if temp.is_safe() {
                allowed += 1;
                break;
            }
        }
    }
    allowed
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    fn input() -> Vec<Vec<i32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }
    #[test]
    fn sample1() {
        assert_eq!(part1(&input()), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input()), 4);
    }
}
