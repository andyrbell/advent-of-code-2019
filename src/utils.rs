use std::fs;

pub(crate) fn read_input<F, T>(file: &str, line_mapper: F) -> Vec<T>
    where F: Fn(&str) -> T {
    let content = fs::read_to_string(format!("inputs/{}.txt", file))
        .expect("error reading input file");

    content.lines()
        .map(line_mapper)
        .collect::<Vec<T>>()
}

fn str_to_int(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

pub(crate) fn split_csv(s: &str) -> Vec<i64> {
    s.split(",")
        .map(str_to_int)
        .collect::<Vec<i64>>()
}
