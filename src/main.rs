use std::time::Instant;

use humantime::format_duration;
use sequencer::builders::vector_with_holes;

pub fn main() {
    let ten: usize = 10;

    for power in 1..10 {
        let length = ten.pow(power);
        println!("Building vector with {} elements...", length);
        let sequence = vector_with_holes(length, 1);

        println!("Running 'divide_and_conquer':");
        let now = Instant::now();
        let (comparisons, number) = sequencer::divide_and_conquer::next_number(sequence.as_slice());
        let elapsed = now.elapsed();
        println!("next number is {}", number);
        println!(
            "took {} comparisons and ran in {:?}",
            comparisons,
            format_duration(elapsed).to_string()
        );
        println!("");

        println!("Running 'sequential_search':");
        let now = Instant::now();
        let (comparisons, number) = sequencer::sequential_search::next_number(sequence.as_slice());
        let elapsed = now.elapsed();
        println!("next number is {}", number);
        println!(
            "took {} comparisons and ran in {:?}",
            comparisons,
            format_duration(elapsed).to_string()
        );
        println!("===============");
    }
}
