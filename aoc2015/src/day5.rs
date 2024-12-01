use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[inline]
fn three_vowels(string: &str, vowels: &HashSet<char>) -> bool {
    string
        .chars()
        .filter(|&c| vowels.contains(&c))
        .count()
        >= 3
}

#[inline]
fn twice_in_a_row(string: &str) -> bool {
    let mut bytes = string.bytes();
    let mut prev = bytes.next();
    bytes.any(|curr| {
        let result = prev == Some(curr);
        prev = Some(curr);
        result
    })
}

#[inline]
fn no_forbidden_strings(string: &str) -> bool {
    let mut bytes = string.bytes();
    let mut prev = bytes.next();
    bytes.all(|curr| {
        let pair = (prev, curr);
        prev = Some(curr);
        !matches!(pair, (Some(b'a'), b'b') | (Some(b'c'), b'd') | (Some(b'p'), b'q') | (Some(b'x'), b'y'))
    })
}

#[inline]
fn is_nice(string: &str, vowels: &HashSet<char>) -> bool {
    three_vowels(string, vowels) && twice_in_a_row(string) && no_forbidden_strings(string)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    input.lines().filter(|l| is_nice(l.trim(), &vowels)).count()
}


fn two_pairs(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || two_pairs(&string[1..])
}

fn repeat_separated(string: &str) -> bool {
    string
        .chars()
        .zip(string.chars().skip(2))
        .any(|(a, b)| a == b)
}

fn is_really_nice(string: &str) -> bool {
    two_pairs(string) && repeat_separated(string)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    input.lines().filter(|l| is_really_nice(l.trim())).count()
}

