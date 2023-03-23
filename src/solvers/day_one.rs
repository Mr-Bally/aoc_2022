use crate::helpers::{file_reader::read_file, integer_parser::parser_int_from_string};

pub fn solve_part_one(path: &str) {
    let file_contents = read_file(path);

    let mut current_max = 0;
    let mut current_total = 0;

    for line in file_contents.split("\r") {
        if line.trim().is_empty() {
            if current_max < current_total {
                current_max = current_total;
            }

            current_total = 0;
        }
        else {
            current_total += parser_int_from_string(line.trim());
        }
    }

    print!("Max value of the set is {}", current_max);
}

pub fn solve_part_two(path: &str) {
    let file_contents = read_file(path);

    let mut totals = vec![];
    let mut current_total = 0;

    for line in file_contents.split("\r") {
        if line.trim().is_empty() {
            totals.push(current_total);
            current_total = 0;
        }
        else {
            current_total += parser_int_from_string(line.trim());
        }
    }

    totals.sort();
    totals.reverse();

    let top_three: i32 = totals[0..3].iter().sum();

    print!("Max value of the top three items is {}", top_three);
}
