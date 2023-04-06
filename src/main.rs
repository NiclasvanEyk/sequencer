use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use humantime::format_duration;
use sequencer::builders::vector_with_holes;
use std::{fmt::Alignment, time::Instant};

struct Row {
    length: usize,
    took_ss: String,
    took_cc: String,
    comparisons_ss: u32,
    comparisons_cc: u32,
}

pub fn main() {
    let ten: usize = 10;

    let mut rows: Vec<Row> = Vec::new();

    for power in 1..10 {
        let length = ten.pow(power);
        println!("Building vector with {} elements...", length);
        let sequence = vector_with_holes(length, 1);

        println!("Running 'divide_and_conquer':");
        let now = Instant::now();
        let (comparisons_cc, _) = sequencer::divide_and_conquer::next_number(sequence.as_slice());
        let elapsed = now.elapsed();
        let took_cc = format_duration(elapsed).to_string();

        println!("Running 'sequential_search':");
        let now = Instant::now();
        let (comparisons_ss, _) = sequencer::sequential_search::next_number(sequence.as_slice());
        let elapsed = now.elapsed();
        let took_ss = format_duration(elapsed).to_string();

        rows.push(Row {
            length,
            took_ss,
            took_cc,
            comparisons_ss,
            comparisons_cc,
        });
        println!("");
    }

    let table = rows
        .iter()
        .map(|row| {
            vec![
                row.length.cell().justify(Justify::Right),
                row.took_cc.clone().cell().justify(Justify::Right),
                row.comparisons_cc.cell().justify(Justify::Right),
                row.took_ss.clone().cell().justify(Justify::Right),
                row.comparisons_ss.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec![
            "Length".cell().bold(true),
            "Took CC".cell().bold(true),
            "Comparisons CC".cell().bold(true),
            "Took SS".cell().bold(true),
            "Comparisons SS".cell().bold(true),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
