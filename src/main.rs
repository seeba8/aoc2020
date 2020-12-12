#![feature(str_split_once)]
#[macro_use]
extern crate lazy_static;

use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

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

    let input = read_to_string("resources/day7.txt").unwrap();
    println!("Day 7 part 1: {:?}", day7::get_number_of_possible_bags(input.as_str(), "shiny gold"));
    println!("Day 7 part 2: {:?}", day7::get_number_of_contained_bags(input.as_str(), "shiny gold"));

    let input = read_to_string("resources/day8.txt").unwrap();
    let instructions = day8::parse_input(input.as_str());
    println!("Day 8 part 1: {:?}", day8::execute(&instructions));
    println!("Day 8 part 2: {:?}", day8::execute2(instructions));

    let input = read_to_string("resources/day9.txt").unwrap();
    let part1 = day9::get_error(input.as_str(), 25);
    println!("Day 9 part 1: {:?}", part1);
    println!("Day 9 part 2: {:?}", day9::get_delta_of_contiguous_set(input.as_str(), part1));

    let input = read_to_string("resources/day10.txt").unwrap();
    println!("Day 10 part 1: {:?}", day10::get_jolt_difference(input.as_str()));
    println!("Day 10 part 2: {:?}", day10::get_number_of_possible_arrangements(input.as_str()));

    let input = std::fs::read_to_string("resources/day11.txt").unwrap();
    println!("Day 11 part 1: {}", day11::get_number_of_occupied_seats_after_stabilisation(input.as_str(), 4, 1));
    println!("Day 11 part 1: {}", day11::get_number_of_occupied_seats_after_stabilisation(input.as_str(), 5, isize::max_value() as usize));
}

