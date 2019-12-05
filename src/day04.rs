use itertools::Itertools;

fn never_decreases(password: &Vec<i32>) -> bool {
    let mut sorted = password.clone();
    sorted.sort();
    password == &sorted
}

fn contains_double(password: &Vec<i32>) -> bool {
    password[1] == password[0] ||
    password[2] == password[1] ||
    password[3] == password[2] ||
    password[4] == password[3] ||
    password[5] == password[4]
}

fn contains_exclusive_double(password: &Vec<i32>) -> bool {
    match password.as_slice() {
        [a,b,c,_,_,_] if a == b && b != c => true,
        [a,b,c,d,_,_] if a != b && b == c && c != d => true,
        [_,a,b,c,d,_] if a != b && b == c && c != d => true,
        [_,_,a,b,c,d] if a != b && b == c && c != d => true,
        [_,_,_,a,b,c] if a != b && b == c => true,
        _ => false
    }
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