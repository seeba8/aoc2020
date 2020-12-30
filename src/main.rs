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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17a;
mod day17b;
mod day18a;
mod day18b;
mod day18;
mod day19;


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

    let input = read_to_string("resources/day11.txt").unwrap();
    println!("Day 11 part 1: {}", day11::get_number_of_occupied_seats_after_stabilisation(input.as_str(), 4, 1));
    println!("Day 11 part 2: {}", day11::get_number_of_occupied_seats_after_stabilisation(input.as_str(), 5, isize::max_value() as usize));

    let input = read_to_string("resources/day12.txt").unwrap();
    println!("Day 12 part 1: {}", day12::get_travel_distance(input.as_str(), false));
    println!("Day 12 part 2: {}", day12::get_travel_distance(input.as_str(), true));

    let input = std::fs::read_to_string("resources/day13.txt").unwrap();
    let (bus, wait) = day13::get_earliest_bus(input.as_str());
    println!("Day 13 part 1: best bus: {}, wait: {} => {}", bus, wait, bus * wait);
    println!("Day 13 part 2: {}", day13::get_timestamp_sequence(input.as_str()));

    let input = std::fs::read_to_string("resources/day14.txt").unwrap();
    let mut decoder = day14::Decoder::new(day14::DecoderVersion::V1);
    decoder.run_programme(input.as_str()).unwrap();
    println!("Day 14 part 1: {}", decoder.get_sum_of_memory());
    let mut decoder = day14::Decoder::new(day14::DecoderVersion::V2);
    decoder.run_programme(input.as_str()).unwrap();
    println!("Day 14 part 2: {}", decoder.get_sum_of_memory());

    println!("Day 15 part 1: {}", day15::Sequence::new(&[20, 9, 11, 0, 1, 2]).nth(2020 - 1).unwrap());
    // Commented out because it would take 1 minute to calculate..-
    // println!("Day 15 part 2: {}", day15::Sequence::new(&[20, 9, 11, 0, 1, 2]).skip(30_000_000-1).next().unwrap());

    let input = std::fs::read_to_string("resources/day16.txt").unwrap();
    println!("Day 16 part 1: {}", day16::get_ticket_scanning_error_rate(input.as_str()).unwrap());
    // Todo: part 2

    let input = std::fs::read_to_string("resources/day17.txt").unwrap();
    let mut grid = day17b::Grid::new(&input, 3, 2..4, 3..4);
    grid.tick_n_times(6);
    println!("Day 17 part 1: {}", grid.count_active_cells());

    let mut grid = day17b::Grid::new(&input, 4, 2..4, 3..4);
    grid.tick_n_times(6);
    println!("Day 17 part 2: {}", grid.count_active_cells());

    let input = std::fs::read_to_string("resources/day18.txt").unwrap();
    let sum: isize = input.lines().map(|line|
        day18::interpreter::Interpreter::new(day18::lexer::Lexer::new(line), true).term().unwrap())
        .sum();
    println!("Day 18 part 1: {}", sum);
    let sum: isize = input.lines().map(|line|
        day18::interpreter::Interpreter::new(day18::lexer::Lexer::new(line), false).term().unwrap())
        .sum();
    println!("Day 18 part 2: {}", sum);

    let input = std::fs::read_to_string("resources/day19.txt").unwrap();
    println!("(Day 19 part 1: {}", day19::get_number_of_matching_messages(&input, false).unwrap());
    println!("(Day 19 part 2: {}", day19::get_number_of_matching_messages(&input, true).unwrap());
}



