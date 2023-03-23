use std::num::ParseIntError;

pub fn parser_int_from_string(input: &str) -> i32 {
    match input.parse::<i32>() {
        Ok(number) => number,
        Err(error) => handle_error(error, input)
    }
}

fn handle_error(error: ParseIntError, input: &str) -> i32 {
    print!("Error when parsing int {}, error {}\n", input, error);
    0
}