#![allow(dead_code)]

const RAW: &str = include_str!("input.txt");
const INGREDIENT_COUNT: usize = count_available_ingredients(RAW);
const RANGE_COUNT: usize = count_fresh_ingredient_ranges(RAW);
const DATABASE: CafeteriaDatabase<INGREDIENT_COUNT, RANGE_COUNT> = CafeteriaDatabase::parse(RAW);
const STATS: CafeteriaStats = DATABASE.get_stats();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Ingredient(usize);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct IngredientRange((Ingredient, Ingredient));

#[derive(Debug, Clone, Copy)]
struct CafeteriaDatabase<const INGREDIENT_COUNT: usize, const RANGE_COUNT: usize> {
    fresh_ingredient_ranges: [IngredientRange; RANGE_COUNT],
    available_ingredients: [Ingredient; INGREDIENT_COUNT],
}

#[derive(Debug, Clone, Copy)]
pub struct CafeteriaStats {
    pub fresh_ingredients: usize,
    pub potential_fresh_ingredients: usize,
}

#[inline]
pub const fn solve() -> CafeteriaStats {
    STATS
}

impl IngredientRange {
    pub const fn contains(&self, ingredient: Ingredient) -> bool {
        ingredient.0 >= self.0.0.0 && ingredient.0 <= self.0.1.0
    }
}

impl<const INGREDIENT_COUNT: usize, const RANGE_COUNT: usize>
    CafeteriaDatabase<INGREDIENT_COUNT, RANGE_COUNT>
{
    pub const fn new(
        available_ingredients: [Ingredient; INGREDIENT_COUNT],
        fresh_ingredient_ranges: [IngredientRange; RANGE_COUNT],
    ) -> Self {
        Self {
            available_ingredients,
            fresh_ingredient_ranges,
        }
    }

    pub const fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut cursor = 0;

        let mut ranges = [IngredientRange((Ingredient(0), Ingredient(0))); RANGE_COUNT];
        let mut r = 0;
        while r < RANGE_COUNT {
            let mut start = 0;
            while cursor < len && bytes[cursor] != b'-' {
                start = start * 10 + (bytes[cursor] - b'0') as usize;
                cursor += 1;
            }
            cursor += 1;

            let mut end = 0;
            while cursor < len && bytes[cursor] != b'\n' {
                end = end * 10 + (bytes[cursor] - b'0') as usize;
                cursor += 1;
            }
            cursor += 1;

            ranges[r] = IngredientRange((Ingredient(start), Ingredient(end)));
            r += 1;
        }

        while cursor < len && bytes[cursor] == b'\n' {
            cursor += 1;
        }

        let mut ingredients = [Ingredient(0); INGREDIENT_COUNT];
        let mut i = 0;
        while i < INGREDIENT_COUNT {
            let mut val = 0;
            while cursor < len && bytes[cursor] != b'\n' {
                val = val * 10 + (bytes[cursor] - b'0') as usize;
                cursor += 1;
            }
            cursor += 1;

            ingredients[i] = Ingredient(val);
            i += 1;
        }

        Self::new(ingredients, ranges)
    }

    pub const fn is_ingredient_fresh(&self, ingredient: Ingredient) -> bool {
        let mut i = 0;
        while i < RANGE_COUNT {
            if self.fresh_ingredient_ranges[i].contains(ingredient) {
                return true;
            }
            i += 1;
        }
        false
    }

    pub const fn count_fresh_ingredients(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < INGREDIENT_COUNT {
            if self.is_ingredient_fresh(self.available_ingredients[i]) {
                count += 1;
            }
            i += 1;
        }
        count
    }

    pub const fn count_potential_fresh_ingredients(&self) -> usize {
        if RANGE_COUNT == 0 {
            return 0;
        }

        let mut ranges = self.fresh_ingredient_ranges;

        let mut i = 0;
        while i < RANGE_COUNT {
            let mut j = 0;
            while j < RANGE_COUNT - 1 - i {
                if ranges[j].0.0.0 > ranges[j + 1].0.0.0 {
                    let temp = ranges[j];
                    ranges[j] = ranges[j + 1];
                    ranges[j + 1] = temp;
                }
                j += 1;
            }
            i += 1;
        }

        let mut count = 0;
        let mut current_start = ranges[0].0.0.0;
        let mut current_end = ranges[0].0.1.0;

        let mut k = 1;
        while k < RANGE_COUNT {
            let next_start = ranges[k].0.0.0;
            let next_end = ranges[k].0.1.0;

            if next_start > current_end {
                count += current_end - current_start + 1;

                current_start = next_start;
                current_end = next_end;
            } else if next_end > current_end {
                current_end = next_end;
            }
            k += 1;
        }

        count += current_end - current_start + 1;
        count
    }

    pub const fn get_stats(&self) -> CafeteriaStats {
        CafeteriaStats {
            fresh_ingredients: self.count_fresh_ingredients(),
            potential_fresh_ingredients: self.count_potential_fresh_ingredients(),
        }
    }
}

const fn count_fresh_ingredient_ranges(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut counter = 0;

    let mut last_was_line = false;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            if last_was_line {
                break;
            }

            counter += 1;
            last_was_line = true;
        } else {
            last_was_line = false;
        }

        i += 1;
    }

    counter
}

const fn count_available_ingredients(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut counter = 0;

    let mut passed_ranges = false;
    let mut last_was_line = false;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            if last_was_line {
                passed_ranges = true;
                last_was_line = false;
                continue;
            }

            last_was_line = true;
            if passed_ranges {
                counter += 1;
            }
        } else {
            last_was_line = false;
        }

        i += 1;
    }

    if last_was_line {
        counter -= 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_RAW: &str = include_str!("input-example.txt");
    const EXAMPLE_INGREDIENT_COUNT: usize = count_available_ingredients(EXAMPLE_RAW);
    const EXAMPLE_RANGE_COUNT: usize = count_fresh_ingredient_ranges(EXAMPLE_RAW);
    const EXAMPLE_DATABASE: CafeteriaDatabase<EXAMPLE_INGREDIENT_COUNT, EXAMPLE_RANGE_COUNT> =
        CafeteriaDatabase::parse(EXAMPLE_RAW);
    const EXAMPLE_STATS: CafeteriaStats = EXAMPLE_DATABASE.get_stats();

    #[test]
    fn test_count_fresh_ingredients_example() {
        assert_eq!(EXAMPLE_STATS.fresh_ingredients, 3);
    }

    #[test]
    fn test_count_fresh_ingredients() {
        assert_eq!(STATS.fresh_ingredients, 509);
    }

    #[test]
    fn test_count_potential_fresh_ingredients_example() {
        assert_eq!(EXAMPLE_STATS.potential_fresh_ingredients, 14);
    }

    #[test]
    fn test_count_potential_fresh_ingredients() {
        assert_eq!(STATS.potential_fresh_ingredients, 336790092076620);
    }
}
