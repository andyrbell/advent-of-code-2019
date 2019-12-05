use itertools::Itertools;

fn never_decreases(password: &Vec<i32>) -> bool {
    let mut sorted = password.clone();
    sorted.sort();
    password == &sorted
}

fn contains_double(password: &Vec<i32>) -> bool {
    password.iter()
        .tuple_windows()
        .any(|(a,b)| a == b)
}

fn contains_exclusive_double(password: &Vec<i32>) -> bool {
    password.iter()
        .map(|digit| digit.to_string())
        .group_by(|digit| digit.clone())
        .into_iter()
        .map(|(_, digit_group)| digit_group.count())
        .any(|group_size| group_size == 2)
}

fn to_digits(n: i32) -> Vec<i32> {
    n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1_tests() {
        assert!(never_decreases(&vec![1,1,1,1,1,1]));
        assert!(!never_decreases(&vec![2,2,3,4,5,0]));
        assert!(contains_double(&vec![1,1,1,1,1,1]));
        assert!(!contains_double(&vec![1,2,3,7,8,9]));
    }

    #[test]
    fn day01_part1() {
        let number_of_matching_passwords = (124075..=580769)
            .map(to_digits)
            .filter(never_decreases)
            .filter(contains_double)
            .count();

        println!("Day 04 part 1: {}", number_of_matching_passwords);
    }

    #[test]
    fn day04_part2_tests() {
        assert!(contains_exclusive_double(&vec![1,1,2,2,3,3]));
        assert!(!contains_exclusive_double(&vec![1,2,3,4,4,4]));
        assert!(contains_exclusive_double(&vec![1,1,1,1,2,2]));
    }

    #[test]
    fn day04_part2() {
        let number_of_matching_passwords = (124075..=580769)
            .map(to_digits)
            .filter(never_decreases)
            .filter(contains_exclusive_double)
            .count();

        println!("Day 04 part 2: {}", number_of_matching_passwords);
    }
}