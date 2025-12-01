pub mod secret_entrance;

fn main() {
    println!("Advent of Code 2025");

    let stats = secret_entrance::solve();
    println!("Day 01 - Secret Entrance:");
    println!("  Stops at 0: {}", stats.stops_at_zero);
    println!("  Total wraps: {}", stats.wraps);
}
