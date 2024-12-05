use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Copy, Clone)]
struct AoC(Option<char>);

impl AoC {
    fn first(c: char) -> Self {
        match c {
            'X' => AoC(Some('X')),
            _ => AoC(None),
        }
    }

    fn first2(c: char) -> Self {
        match c {
            'A' => AoC(Some('A')),
            _ => AoC(None),
        }
    }

    fn next(&self) -> Self {
        match self.0 {
            Some('X') => AoC(Some('M')),
            Some('M') => AoC(Some('A')),
            Some('A') => AoC(Some('S')),
            Some('S') => AoC(None),
            None => AoC(None),
            _ => panic!("Invalid input"),
        }
    }

    fn get(&self) -> Option<char> {
        self.0
    }

    fn last(&self) -> bool {
        self.0 == Some('S')
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];
const CROSS: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

#[aoc_generator(day4)]
fn preprocess(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let (rows, cols) = (input.len(), input[0].len());
    let mut stack: Vec<(usize, usize, AoC, Option<(isize, isize)>)> = vec![];
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if AoC::first(input[i][j]).get() == Some('X') {
                stack.push((i, j, AoC::first(input[i][j]), None));
            }
        }
    }
    while let Some((row, col, c, direction)) = stack.pop() {
        let (row, col) = (row as isize, col as isize);
        if c.last() {
            count += 1;
            continue;
        }
        let next = c.next();
        if let (Some(dir), Some(n)) = (direction, next.get()) {
            if 0 <= row + dir.0
                && row + dir.0 < rows as isize
                && 0 <= col + dir.1
                && col + dir.1 < cols as isize
                && n == input[(row + dir.0) as usize][(col + dir.1) as usize]
            {
                stack.push((
                    (row + dir.0) as usize,
                    (col + dir.1) as usize,
                    next,
                    Some(dir),
                ));
            }
            continue;
        }
        for (dx, dy) in DIRECTIONS.iter() {
            if 0 <= row + dx
                && row + dx < rows as isize
                && 0 <= col + dy
                && col + dy < cols as isize
            {
                if let Some(n) = next.get() {
                    if input[(row + dx) as usize][(col + dy) as usize] == n {
                        stack.push((
                            (row + dx) as usize,
                            (col + dy) as usize,
                            next,
                            Some((*dx, *dy)),
                        ));
                    }
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut count = 0;
    let (rows, cols) = (input.len(), input[0].len());
    let mut stack = vec![];
    for i in 0..rows {
        for j in 0..cols {
            if AoC::first2(input[i][j]).get() == Some('A') {
                stack.push((i, j));
            }
        }
    }
    while let Some((row, col)) = stack.pop() {
        let mut ms = vec![];
        let mut as_ = vec![];
        for (dx, dy) in CROSS.iter() {
            if 0 <= row as isize + dx
                && row as isize + dx < rows as isize
                && 0 <= col as isize + dy
                && col as isize + dy < cols as isize
            {
                match input[(row as isize + dx) as usize][(col as isize + dy) as usize] {
                    'M' => ms.push((row as isize + dx, col as isize + dy)),
                    'A' => as_.push((row as isize + dx, col as isize + dy)),
                    _ => {}
                }
                if ms.len() == as_.len() && ms.len() == 2 {
                    if ms[0].0 == ms[1].0 || ms[0].1 == ms[1].1 {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Vec<char>> {
        vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]
    }
    #[test]
    fn test_part1() {
        let input = sample();
        assert_eq!(part1(&input), 18);
    }

    // #[test]
    // fn test_part2(){
    //     let input = sample();
    //     assert_eq!(part2(&input), 9);
    // }
}
