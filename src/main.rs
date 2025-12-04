use std::time::Instant;

mod gift_shop;
mod lobby;
mod secret_entrance;

fn main() {
    println!("🎄 Advent of Code 2025 🎄");
    println!();

    run_day(1, "Secret Entrance", || {
        let stats = secret_entrance::solve();
        println!("  • Part 1 (Stops at 0):  {}", stats.stops_at_zero);
        println!("  • Part 2 (Wraps): {}", stats.wraps);
    });

    run_day(2, "Gift Shop", || {
        let res = gift_shop::solve();
        println!("  • Part 1 (Identical): {}", res.identical_halves_sum);
        println!("  • Part 2 (Repeating): {}", res.repeating_patterns_sum);
    });

    run_day(3, "Lobby", || {
        let power = lobby::solve();
        println!("  • Part 1 (Max 2):  {}", power.max_2);
        println!("  • Part 2 (Max 12): {}", power.max_12);
    });
}

fn run_day<F>(day: u8, title: &str, logic: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    let width = 60;

    println!("┏{}┓", "━".repeat(width - 2));
    println!("┃  Day {:02} • {:<46} ┃", day, title);
    println!("┗{}┛", "━".repeat(width - 2));

    println!();
    logic();
    println!();

    let elapsed = start.elapsed();
    println!("╍{}╍", "╍".repeat(width - 2));
    println!("   ⏱️  {:.2?}\n", elapsed);
}
