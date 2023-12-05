use core::num;
use regex::Regex;
use std::{
    collections::{btree_map::Range, HashMap},
    fs,
    num::ParseIntError,
};

fn main() {
    let mut total = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        parse_number_names(line);
        total += parse_numbers(line);
    }

    println!("Total: {}", total);
}

fn parse_number_names(input: &str) -> usize {
    let num_dict: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let input = "threefourtwo9";

    let mut result = input.clone().to_owned();
    println!("Transformando string '{}'", result);

    let re: Regex =
        Regex::new("(one|two|three|four|five|six|seven|eight|nine|zero|[0-9])").unwrap();

    let b: Vec<Option<usize>> = re
        .find_iter(input)
        .map(|m: regex::Match<'_>| {
            if (m.len() == 1) {
                return num_dict.get(m.as_str());
            }

            Option::from(m.as_str().parse::<usize>().unwrap().to_owned())
        })
        .collect();

    println!("{:?}", b);

    42
}

fn parse_numbers(input: &str) -> usize {
    let re = Regex::new("\\D").unwrap();
    let result = re.replace_all(input, "").to_string();

    let first = &result.chars().next().unwrap();
    let last = &result.chars().last().unwrap();

    format!("{}{}", &first, &last).parse().unwrap()
}
