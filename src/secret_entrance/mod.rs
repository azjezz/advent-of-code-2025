use std::str::FromStr;

const STARTING_DIAL_POSITION: usize = 50;
const INPUT_TEXT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DialStats {
    pub stops_at_zero: usize,
    pub wraps: usize,
}

#[inline]
#[must_use]
pub fn solve() -> DialStats {
    run_simulation(STARTING_DIAL_POSITION, INPUT_TEXT)
}

#[must_use]
fn run_simulation(start_pos: usize, input: &str) -> DialStats {
    let mut dial = Dial::new(start_pos, 100);
    let mut stats = DialStats::default();

    let rotations = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Command>().expect("invalid input format"));

    for rotation in rotations {
        stats.wraps += dial.turn(rotation);

        if dial.position() == 0 {
            stats.stops_at_zero += 1;
        }
    }

    stats
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
struct Dial {
    position: usize,
    circumference: usize,
}

impl Dial {
    pub fn new(position: usize, dial_size: usize) -> Self {
        Self {
            position,
            circumference: dial_size,
        }
    }

    pub fn turn(&mut self, command: Command) -> usize {
        let start = self.position;
        let size = self.circumference;

        match command {
            Command::Right(amount) => {
                self.position = (start + amount) % size;
                (start + amount) / size
            }
            Command::Left(amount) => {
                let effective_move = amount % size;
                self.position = (start + size - effective_move) % size;

                let inverted_start = (size - start) % size;
                (inverted_start + amount) / size
            }
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("empty command string");
        }

        let (direction, amount_str) = s.split_at(1);
        let amount = amount_str.parse::<usize>().map_err(|_| "invalid amount")?;

        match direction {
            "R" | "r" => Ok(Command::Right(amount)),
            "L" | "l" => Ok(Command::Left(amount)),
            _ => Err("unknown direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_count_example() {
        let stats = run_simulation(50, "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");

        assert_eq!(3, stats.stops_at_zero);
    }

    #[test]
    fn test_zero_count() {
        let stats = solve();

        assert_eq!(1018, stats.stops_at_zero);
    }

    #[test]
    fn test_full_rotations_example() {
        let stats = run_simulation(50, "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");

        assert_eq!(6, stats.wraps);
    }

    #[test]
    fn test_full_rotations() {
        let stats = solve();

        assert_eq!(5815, stats.wraps);
    }
}
