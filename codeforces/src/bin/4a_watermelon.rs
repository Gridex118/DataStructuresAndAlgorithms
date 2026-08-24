use std::io;

/// Prints *YES*, if watermelon can be divided into two parts, each of
/// them weighting even number of kilos; and *NO* otherwise
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)
        .expect("Failed to read input");
    let weight: i32 = input_line.trim()
        .parse()
        .expect("Failed to parse i32");
    let can_split_evenly = weight > 2 && weight % 2 == 0;
    println!("{}", if can_split_evenly { "YES" } else { "NO" });
}
