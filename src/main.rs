#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = read_to_string("resources/day1.txt").unwrap();
    println!("Day 1 part 1: {:?}", day1::day1a(input.as_str()));
    println!("Day 1 part 2: {:?}", day1::day1b(input.as_str()));

    let input = read_to_string("resources/day2.txt").unwrap();
    println!("Day 2 part 1: {:?}", day2::count_valid_passwords(input.as_str()));
    println!("Day 2 part 2: {:?}", day2::count_valid_passwords_new_policy(input.as_str()));

    let input = read_to_string("resources/day3.txt").unwrap();
    println!("Day 3 part 1: {:?}", day3::slide_down(input.as_str(), 3, 1));
    println!("Day 3 part 2: {:?}", day3::multiply_slopes(input.as_str()));

    let input = read_to_string("resources/day4.txt").unwrap();
    println!("Day 4 part 1: {:?}", day4::count_valid_passports(input.as_str(), false));
    println!("Day 4 part 2: {:?}", day4::count_valid_passports(input.as_str(), true));

    let input = read_to_string("resources/day5.txt").unwrap();
    println!("Day 5 part 1: {:?}", day5::get_highest_id(input.as_str()));
    println!("Day 5 part 2: {:?}", day5::get_missing_id(input.as_str()));

    let input = read_to_string("resources/day6.txt").unwrap();
    println!("Day 6 part 1: {:?}", day6::get_sum_of_distinct_answers(input.as_str()));
    println!("Day 6 part 2: {:?}", day6::get_sum_of_common_answers(input.as_str()));
}
