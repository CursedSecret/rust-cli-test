mod kaprekar;
use kaprekar::kaprekar::kaprekar;

mod kaprekar_tests;

use std::time::Instant;

fn main() {
    let iterations_max = 100;

    let start = Instant::now();
    for _iterations in 1..iterations_max {
        for number in 1..9999 {
            kaprekar(number);
            // println!("Kaprekar of {} = {}", number, kaprekar(number));
        }
    }
    println!(
        "Took {:?} to run {} iterations of kaprekar",
        Instant::now() - start,
        iterations_max
    );
}
