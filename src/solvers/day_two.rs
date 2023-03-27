use crate::helpers::{file_reader::read_file};

pub fn solve(path: &str) {
    let file_contents = read_file(path);
    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for line in file_contents.split("\r") {
        part_one_total += calculate_score_part_one(line.trim());
        part_two_total += calculate_score_part_two(line.trim());
    }

    print!("Total for part one is {} and for part two {}", part_one_total, part_two_total);
}

fn calculate_score_part_one(line: &str) -> i32 {
    let char_vec: Vec<char> = line.chars().collect();
    let choice_one = char_vec.first().unwrap();
    let choice_two = char_vec.last().unwrap();

    let mut score = calculate_win_total_part_one(choice_one, choice_two);
    score += calculate_choice_score_part_one(choice_two);

    return score;
}

// Win = 6, draw = 3, loss = 0
fn calculate_win_total_part_one(opt_one: &char, opt_two: &char) -> i32 {
    match (opt_one, opt_two) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => return 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => return 6,
        _ => return 0
    }
}

// A = Rock, B = Paper, C = Scissors
// X = Rock, Y = Paper, Z = Scissors
fn calculate_choice_score_part_one(choice: &char) -> i32 {
    match choice {
        'X' => return 1,
        'Y' => return 2,
        _ => return 3
    }
}

fn calculate_score_part_two(line: &str) -> i32 {
    let char_vec: Vec<char> = line.chars().collect();
    let op_choice = char_vec.first().unwrap();
    let result = char_vec.last().unwrap();

    let mut score = calculate_win_total_part_two(result);
    score += calculate_choice_total_part_two(result, op_choice);

    return score;
}

// X = 0, Y = 3, Z = 6
fn calculate_win_total_part_two(result: &char) -> i32 {
    match result {
        'X' => return 0,
        'Y' => return 3,
        _ => return 6
    }
}

fn calculate_choice_total_part_two(result: &char, op_choice: &char) -> i32 {
    match result {
        'X' => return get_losing_choice_part_two(op_choice),
        'Y' => return choice_value_part_two(op_choice),
        _ => return get_winning_choice_part_two(op_choice)
    }
}

fn choice_value_part_two(result: &char) -> i32 {
    match result {
        'A' => return 1,
        'B' => return 2,
        _ => return 3
    }
}

fn get_winning_choice_part_two(choice: &char) -> i32 {
    match choice {
        'A' => return choice_value_part_two(&'B'),
        'B' => return choice_value_part_two(&'C'),
        _ => return choice_value_part_two(&'A')
    }
}

fn get_losing_choice_part_two(choice: &char) -> i32 {
    match choice {
        'A' => return choice_value_part_two(&'C'),
        'B' => return choice_value_part_two(&'A'),
        _ => return choice_value_part_two(&'B')
    }
}
