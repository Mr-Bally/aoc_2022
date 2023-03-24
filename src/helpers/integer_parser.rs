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

mod test {
    use crate::helpers::integer_parser::parser_int_from_string;

    #[test]
    fn parser_work_for_integer() {
        let expected = 30;
        let actual = parser_int_from_string("30");
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_returns_zero_for_letters() {
        let expected = 0;
        let actual = parser_int_from_string("ThisWontWork");
        
        assert_eq!(expected, actual);
    }
}
