use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Mul {
    a: i32,
    b: i32,
}

impl Mul {
    pub fn from_str(s: &str) -> Option<Mul> {
        if s.starts_with("mul(") && s.ends_with(")") {
            let content = &s[4..s.len() - 1];
            if let [a, b] = content.split(',').collect::<Vec<&str>>()[..] {
                match (a.parse(), b.parse()) {
                    (Ok(a), Ok(b)) => return Some(Mul { a, b }),
                    _ => return None,
                }
            }
        }
        None
    }
    fn eval(&self) -> i32 {
        self.a * self.b
    }
}

struct State(i32);

impl State {
    fn new() -> Self {
        State(0)
    }
    fn next(&mut self) {
        self.0 += 1;
    }

    fn clear(&mut self) {
        self.0 = 0;
    }

    fn get(&self) -> i32 {
        self.0
    }

    fn set(&mut self, state: i32) {
        self.0 = state;
    }
}

struct Command {
    state: State,
    enable: bool,
}

impl Command {
    fn new(enable: bool) -> Self {
        Command {
            state: State(0),
            enable,
        }
    }

    fn process(&mut self, c: char) {
        match (self.state.get(), c) {
            (0, 'd') | (1, 'o') | (2, '(') | (4, '\'') | (5, 't') | (6, '(') => self.state.next(),
            (3, ')') => {
                self.state.clear();
                self.enable = true;
            }
            (7, ')') => {
                self.state.clear();
                self.enable = false;
            }
            (2, 'n') => self.state.set(4),
            _ => self.state.clear(),
        }
    }
}

struct Automata {
    state: State,
    buffer: String,
    command: Option<Command>,
}

impl Automata {
    pub fn new(enable: impl Into<Option<bool>>) -> Self {
        Self {
            state: State::new(),
            buffer: String::new(),
            command: enable.into().map(Command::new),
        }
    }

    fn process(&mut self, c: char) -> Option<String> {
        if let Some(command) = &mut self.command {
            command.process(c);
        }
        match (self.state.get(), c) {
            (0, 'm') | (1, 'u') | (2, 'l') | (3, '(') | (5, ',') => self.next(c),
            (4, c) | (6, c) if c.is_ascii_digit() => self.next(c),
            (5, c) | (7, c) if c.is_ascii_digit() => self.stay(c),
            (7, ')') => {
                self.next(c);
                let buffer = self.buffer.clone();
                self.clear();
                match &self.command {
                    Some(c) if c.enable => return Some(buffer),
                    None => return Some(buffer),
                    _ => return None,
                }
            }
            _ => self.clear(),
        };

        None
    }
    fn read(&mut self, s: &str) -> Vec<Mul> {
        let mut muls = Vec::new();
        for c in s.chars() {
            if let Some(formation) = self.process(c) {
                if let Some(mul) = Mul::from_str(&formation) {
                    muls.push(mul);
                }
            }
        }
        muls
    }

    fn next(&mut self, c: char) {
        self.state.next();
        self.buffer.push(c);
    }

    fn clear(&mut self) {
        self.state.clear();
        self.buffer.clear();
    }

    fn stay(&mut self, c: char) {
        self.buffer.push(c);
    }
}

#[aoc_generator(day3)]
fn preprocess(input: &str) -> String {
    input.replace("\n", "")
}

#[aoc(day3, part1)]
fn part1(gibberish: &str) -> i32 {
    let mut automata = Automata::new(None);
    let muls = automata.read(gibberish);
    muls.iter().map(|mul| mul.eval()).sum()
}

#[aoc(day3, part2)]
fn part2(gibberish: &str) -> i32 {
    let mut automata = Automata::new(true);
    let muls = automata.read(gibberish);
    muls.iter().map(|mul| mul.eval()).sum()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    pub fn sample1() {
        let gibberish = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)))";
        assert_eq!(part1(&gibberish.to_string()), 161);
    }

    #[test]
    pub fn sample2() {
        let gibberish = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&gibberish.to_string()), 48);
    }
}
