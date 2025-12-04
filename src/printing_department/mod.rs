const RAW: &str = include_str!("input.txt");
const WIDTH: usize = calculate_width(RAW) + 2;
const HEIGHT: usize = calculate_height(RAW) + 2;
const GRID: Grid<HEIGHT, WIDTH> = Grid::parse(RAW);

const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone, Copy)]
struct Grid<const H: usize, const W: usize> {
    cells: [[bool; W]; H],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridStats {
    pub accessible: usize,
    pub total_removable: usize,
}

pub fn solve() -> GridStats {
    run_simulation(GRID)
}

fn run_simulation<const H: usize, const W: usize>(mut grid: Grid<H, W>) -> GridStats {
    let mut queue = Vec::with_capacity((H * W) / 4);

    let mut queued = [[false; W]; H];
    // for y in 1..H - 1 {
    //     for x in 1..W - 1 {
    //         if grid.cells[y][x] && grid.is_accessible(x, y) {
    //             grid.cells[y][x] = false;
    //             queued[y][x] = true;
    //             queue.push((x, y));
    //         }
    //     }
    // }
    for (y, row) in grid.cells.iter().enumerate().skip(1).take(H - 2) {
        for (x, filled) in row.iter().enumerate().skip(1).take(W - 2) {
            if *filled && grid.is_accessible(x, y) {
                queued[y][x] = true;
                queue.push((x, y));
            }
        }
    }

    let accessible = queue.len();
    for &(x, y) in &queue {
        grid.cells[y][x] = false;
    }

    let mut head = 0;
    while head < queue.len() {
        let (cx, cy) = queue[head];
        head += 1;

        for &(dx, dy) in &ADJACENT_OFFSETS {
            let nx = (cx as isize + dx) as usize;
            let ny = (cy as isize + dy) as usize;

            if grid.cells[ny][nx] && !queued[ny][nx] && grid.is_accessible(nx, ny) {
                grid.cells[ny][nx] = false;
                queued[ny][nx] = true;
                queue.push((nx, ny));
            }
        }
    }

    GridStats {
        accessible,
        total_removable: queue.len(),
    }
}

impl<const H: usize, const W: usize> Grid<H, W> {
    pub const fn parse(input: &str) -> Self {
        let mut cells = [[false; W]; H];
        let bytes = input.as_bytes();
        let mut i = 0;
        let mut x = 1;
        let mut y = 1;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'\n' {
                y += 1;
                x = 1;
            } else if b != b'\r' && x < W - 1 && y < H - 1 {
                cells[y][x] = b == b'@';
                x += 1;
            }

            i += 1;
        }

        Self { cells }
    }

    pub fn is_accessible(&self, x: usize, y: usize) -> bool {
        let mut filled_adjacent = 0;

        for &(dx, dy) in &ADJACENT_OFFSETS {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if self.cells[ny][nx] {
                filled_adjacent += 1;
                if filled_adjacent >= 4 {
                    return false;
                }
            }
        }

        true
    }
}

const fn calculate_width(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            return i;
        }

        i += 1;
    }

    i
}

const fn calculate_height(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut lines = 0;
    let mut has_chars = false;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            lines += 1;
            has_chars = false;
        } else {
            has_chars = true;
        }
        i += 1;
    }

    if has_chars { lines + 1 } else { lines }
}

// debugging helper to print the grid state
#[allow(dead_code)]
fn debug_grid<const H: usize, const W: usize>(grid: &Grid<H, W>) {
    for y in 0..H {
        for x in 0..W {
            let is_padding = x == 0 || y == 0 || x == W - 1 || y == H - 1;

            if grid.cells[y][x] && !is_padding {
                if grid.is_accessible(x, y) {
                    print!("X");
                } else {
                    print!("@");
                }
            } else if is_padding {
                print!("~");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_RAW: &str = include_str!("example-input.txt");
    const EXAMPLE_WIDTH: usize = calculate_width(EXAMPLE_RAW) + 2;
    const EXAMPLE_HEIGHT: usize = calculate_height(EXAMPLE_RAW) + 2;
    const EXAMPLE_GRID: Grid<EXAMPLE_HEIGHT, EXAMPLE_WIDTH> = Grid::parse(EXAMPLE_RAW);

    #[test]
    fn test_accessible_example() {
        let stats = run_simulation(EXAMPLE_GRID);

        assert_eq!(stats.accessible, 13);
    }

    #[test]
    fn test_accessible() {
        let stats = run_simulation(GRID);

        assert_eq!(stats.accessible, 1604);
    }

    #[test]
    fn test_total_removable_example() {
        let stats = run_simulation(EXAMPLE_GRID);

        assert_eq!(stats.total_removable, 43);
    }

    #[test]
    fn test_total_removable() {
        let stats = run_simulation(GRID);

        assert_eq!(stats.total_removable, 9397);
    }
}
