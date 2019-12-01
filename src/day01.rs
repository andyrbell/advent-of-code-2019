use std::ops::{Div, Sub, Add};
use std::fs;
use std::cmp::max;

fn solve_part1(input: Vec<i64>) -> i64 {
    input.iter()
        .map(|mass| mass.div(3).sub(2))
        .sum()
}

fn solve_part2(input: Vec<i64>) -> i64 {
    input.iter()
        .map(|mass| calculate_fuel(0, *mass))
        .sum()
}

fn calculate_fuel(acc: i64, mass: i64) -> i64 {
    match mass > 0 {
        false => acc,
        true => {
            let fuel = mass.div(3).sub(2);
            calculate_fuel(acc.add(max(fuel, 0)), fuel)
        }
    }
}

fn read_input() -> Vec<i64> {
    let content = fs::read_to_string("inputs/Day01.txt")
        .expect("error reading input file");

    content.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_part1_test() {
        assert_eq!(solve_part1(vec![12]), 2);
        assert_eq!(solve_part1(vec![14]), 2);
        assert_eq!(solve_part1(vec![1969]), 654);
        assert_eq!(solve_part1(vec![100756]), 33583);
    }

    #[test]
    fn day01_part1() {
        println!("Day 1 part 1: {}", solve_part1(read_input()));
    }
    
    #[test]
    fn day01_part2_test() {
        assert_eq!(solve_part2(vec![14]), 2);
        assert_eq!(solve_part2(vec![1969]), 966);
        assert_eq!(solve_part2(vec![100756]), 50346);
    }

    #[test]
    fn day01_part2() {
        println!("Day 1 part 2: {}", solve_part2(read_input()));
    }
}