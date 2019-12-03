use std::fs;
use crate::utils::read_input;
use crate::utils::split_csv;

fn solve_part1(input: Vec<i64>) -> Vec<i64> {
    run_program(0,input)
}

fn run_program(pointer: i32, mut opcodes: Vec<i64>) -> Vec<i64> {
    match opcodes.get(pointer as usize) {
        Some(99) => opcodes,
        Some(1) => {
            add(pointer, &mut opcodes);
            run_program(pointer + 4, opcodes)
        },
        Some(2) => {
            multiply(pointer, &mut opcodes);
            run_program(pointer + 4, opcodes)
        },
        _ => panic!("unknown operation")
    }
}

fn multiply(pointer: i32, opcodes: &mut Vec<i64>) {
    let arg1_index = opcodes[(pointer + 1) as usize];
    let arg2_index = opcodes[(pointer + 2) as usize];
    let arg1_value = opcodes[arg1_index as usize];
    let arg2_value = opcodes[arg2_index as usize];
    let output_index = opcodes[(pointer + 3) as usize];
    opcodes[output_index as usize] = arg1_value * arg2_value;
}

fn add(pointer: i32, opcodes: &mut Vec<i64>) {
    let arg1_index = opcodes[(pointer + 1) as usize];
    let arg2_index = opcodes[(pointer + 2) as usize];
    let arg1_value = opcodes[arg1_index as usize];
    let arg2_value = opcodes[arg2_index as usize];
    let output_index = opcodes[(pointer + 3) as usize];
    opcodes[output_index as usize] = arg1_value + arg2_value;
}

fn solve_part2(input: &Vec<i64>, noun: i64, verb: i64) -> Vec<i64> {
    let mut input = input.clone();
    input[1] = noun;
    input[2] = verb;
    solve_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_tests() {
        assert_eq!(solve_part1(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(solve_part1(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(solve_part1(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(solve_part1(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn day02_part1() {
        let lines = read_input("Day02", split_csv);
        let mut input = lines[0].clone();
        input[1] = 12;
        input[2] = 2;

        println!("Day 2 part 1: {}", solve_part1(input).get(0).unwrap());
    }

    #[test]
    fn day02_part2() {
        let lines = read_input("Day02", split_csv);
        let mut input = lines[0].clone();

        (0..100).for_each(|noun|
            (0..100).for_each(|verb| {
                let result = solve_part2(&input, noun, verb);
                if result[0] == 19690720 {
                    println!("Day 2 part 2: {}", 100 * noun + verb);
                }
            })
        )
    }
}