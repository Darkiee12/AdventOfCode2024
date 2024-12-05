use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug)]
struct Page {
    value: i32,
    from: HashSet<i32>,
    to: HashSet<i32>,
}

impl Page {
    fn new(value: i32) -> Self {
        Self {
            value,
            from: HashSet::new(),
            to: HashSet::new(),
        }
    }

    fn insert_to(&mut self, page: i32) {
        self.to.insert(page);
    }

    fn insert_from(&mut self, page: i32) {
        self.from.insert(page);
    }
}

#[derive(Debug)]
struct PageUpdate(HashMap<i32, Page>);
impl PageUpdate {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, from: i32, to: i32) {
        if let Some(page) = self.0.get_mut(&from) {
            page.insert_to(to);
        } else {
            let mut page = Page::new(from);
            page.insert_to(to);
            self.0.insert(from, page);
        }

        if let Some(page) = self.0.get_mut(&to) {
            page.insert_from(from);
        } else {
            let mut page = Page::new(to);
            page.insert_from(from);
            self.0.insert(to, page);
        }
    }

    fn insert_many(&mut self, iter: impl Iterator<Item = (i32, i32)>) {
        for (from, to) in iter {
            self.insert(from, to);
        }
    }

    fn follows_topological_order(&self, order: &[i32]) -> bool {
        order.windows(2).all(|window| {
            self.0
                .get(&window[1])
                .map(|page| page.from.contains(&window[0]))
                .unwrap_or(false)
        })
    }

    fn get_topological_order(&self, list: &[i32]) -> Vec<i32> {
        let mut pages: Vec<&Page> = list.iter().map(|&page| &self.0[&page]).collect();

        pages.sort_by(|a, b| {
            if a.from.get(&b.value).is_some() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        pages.iter().map(|page| page.value).collect()
    }
}
#[aoc_generator(day5)]
pub fn preprocess(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (first, second) = input.split_once("\n\n").unwrap();

    let part1 = first
        .lines()
        .map(|line| {
            let mut iter = line.split("|");
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let part2 = second
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (part1, part2)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (pages, tests) = input;
    let mut page_update = PageUpdate::new();
    page_update.insert_many(pages.iter().cloned());
    tests
        .iter()
        .filter(|test| page_update.follows_topological_order(test))
        .map(|valid| valid[valid.len() >> 1])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let (pages, tests) = input;
    let mut page_update = PageUpdate::new();
    page_update.insert_many(pages.iter().cloned());
    tests
        .iter()
        .filter(|test| !page_update.follows_topological_order(test))
        .map(|invalid| page_update.get_topological_order(invalid))
        .map(|valid| valid[valid.len() >> 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        preprocess(data)
    }

    #[test]
    fn test_part1() {
        let (pages, tests) = input();
        assert_eq!(part1(&(pages, tests)), 143);
    }

    #[test]
    fn test_part2() {
        let (pages, tests) = input();
        assert_eq!(part2(&(pages, tests)), 123);
    }
}
