use std::str::FromStr;

const INPUT_TEXT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Battery {
    pub joltage: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BatteryBank {
    pub batteries: Vec<Battery>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BatteryPower {
    pub max_2: usize,
    pub max_12: usize,
}

#[inline]
#[must_use]
pub fn solve() -> BatteryPower {
    run_simulation(INPUT_TEXT)
}

#[must_use]
fn run_simulation(input: &str) -> BatteryPower {
    let banks = BatteryBank::parse_all(input);

    let mut power = BatteryPower::default();
    for bank in &banks {
        let bank_power = bank.calculate_power();

        power.max_2 += bank_power.max_2;
        power.max_12 += bank_power.max_12;
    }

    power
}

impl BatteryBank {
    pub fn parse_all(input: &str) -> Vec<Self> {
        input.lines().filter_map(|line| line.parse().ok()).collect()
    }

    pub fn calculate_power(&self) -> BatteryPower {
        let values: Vec<u8> = self.batteries.iter().map(|b| b.joltage).collect();

        let max_2_vec = Self::max_subsequence(&values, 2);
        let max_12_vec = Self::max_subsequence(&values, 12);

        BatteryPower {
            max_2: Self::vec_to_usize(&max_2_vec),
            max_12: Self::vec_to_usize(&max_12_vec),
        }
    }

    fn max_subsequence(values: &[u8], k: usize) -> Vec<u8> {
        let n = values.len();
        if k > n {
            return vec![];
        }

        let mut drop_budget = n - k;
        let mut stack: Vec<u8> = Vec::with_capacity(k);

        for val in values {
            while drop_budget > 0 {
                if let Some(&top) = stack.last() {
                    if *val > top {
                        stack.pop();
                        drop_budget -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            stack.push(*val);
        }

        stack.truncate(k);
        stack
    }

    fn vec_to_usize(digits: &[u8]) -> usize {
        digits.iter().fold(0, |acc, &d| (acc * 10) + d as usize)
    }
}

impl FromStr for BatteryBank {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| Battery { joltage: d as u8 }))
            .collect();

        Ok(BatteryBank { batteries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_2_example() {
        let power = run_simulation(
            "
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        ",
        );

        assert_eq!(power.max_2, 357);
    }

    #[test]
    fn test_max_2() {
        let power = run_simulation(INPUT_TEXT);
        assert_eq!(power.max_2, 17403);
    }

    #[test]
    fn test_max_12_example() {
        let power = run_simulation(
            "
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        ",
        );

        assert_eq!(power.max_12, 3121910778619);
    }

    #[test]
    fn test_max_12() {
        let power = run_simulation(INPUT_TEXT);
        assert_eq!(power.max_12, 173416889848394);
    }
}
