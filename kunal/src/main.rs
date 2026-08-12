use kunal::patterns::*;

fn main() {
    println!("Full Diamond Pattern with 15 rows");
    println!("{}\n", pattern_full_diamond(15).unwrap());
    println!("Number Square Pattern with max number 5");
    println!("{}\n", pattern_number_square(5));
    println!("Top Hald Diamond Number Pattern with 9 rows");
    println!("{}", pattern_top_half_diamond_number_symmetric(9));
}
