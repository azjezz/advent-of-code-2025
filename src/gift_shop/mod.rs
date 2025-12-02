use std::ops::AddAssign;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default, Clone, Copy)]
pub struct InvalidProductIds {
    pub identical_halves_sum: usize,
    pub repeating_patterns_sum: usize,
}

impl AddAssign for InvalidProductIds {
    fn add_assign(&mut self, rhs: Self) {
        self.identical_halves_sum += rhs.identical_halves_sum;
        self.repeating_patterns_sum += rhs.repeating_patterns_sum;
    }
}

#[inline]
#[must_use]
pub fn solve() -> InvalidProductIds {
    run_simulation(INPUT)
}

#[must_use]
fn run_simulation(input: &str) -> InvalidProductIds {
    let mut total = InvalidProductIds::default();

    for (start, end) in parse_ranges(input) {
        total += calculate_range_metrics(start, end);
    }

    total
}

fn calculate_range_metrics(start: usize, end: usize) -> InvalidProductIds {
    let mut metrics = InvalidProductIds::default();

    if start > end {
        return metrics;
    }

    let min_digits = if start == 0 {
        1
    } else {
        start.ilog10() as usize + 1
    };
    let max_digits = if end == 0 {
        1
    } else {
        end.ilog10() as usize + 1
    };

    let mut found_patterns = Vec::new();
    for len in min_digits..=max_digits {
        for parts in 2..=len {
            if len % parts != 0 {
                continue;
            }

            let chunk_len = len / parts;
            let multiplier = generate_repetition_multiplier(chunk_len, parts);

            let seed_min_digits = 10usize.pow(chunk_len as u32 - 1);
            let seed_max_digits = 10usize.pow(chunk_len as u32) - 1;

            let seed_start = start.div_ceil(multiplier);
            let seed_end = end / multiplier;

            let valid_start = seed_start.max(seed_min_digits);
            let valid_end = seed_end.min(seed_max_digits);

            if valid_start > valid_end {
                continue;
            }

            for seed in valid_start..=valid_end {
                let n = seed * multiplier;

                found_patterns.push(n);

                if parts == 2 {
                    metrics.identical_halves_sum += n;
                }
            }
        }
    }

    found_patterns.sort_unstable();
    found_patterns.dedup();
    metrics.repeating_patterns_sum = found_patterns.iter().sum();

    metrics
}

#[inline(always)]
fn generate_repetition_multiplier(len: usize, count: usize) -> usize {
    let mut multiplier = 0;
    let step = 10usize.pow(len as u32);
    let mut current = 1;

    for _ in 0..count {
        multiplier += current;
        current *= step;
    }

    multiplier
}

fn parse_ranges(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.split(',').filter_map(|part| {
        let (s, e) = part.split_once('-')?;
        let start = s.trim().parse().ok()?;
        let end = e.trim().parse().ok()?;
        Some((start, end))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_halves_sum_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = run_simulation(input);

        assert_eq!(result.identical_halves_sum, 1_227_775_554);
    }

    #[test]
    fn test_identical_halves_sum() {
        let result = solve();

        assert_eq!(result.identical_halves_sum, 31_210_613_313);
    }

    #[test]
    fn test_repeating_patterns_sum_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = run_simulation(input);

        assert_eq!(result.repeating_patterns_sum, 4_174_379_265);
    }

    #[test]
    fn test_repeating_patterns_sum() {
        let result = solve();

        assert_eq!(result.repeating_patterns_sum, 41_823_587_546);
    }
}
