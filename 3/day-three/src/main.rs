use regex::{Match, Regex};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let matches: Vec<LineMatch> = file.lines().map(|line| get_symbols(line)).collect();
    matches.iter().fold(0, |val, m| val + get_sum_of_valid(m));
}

fn get_sum_of_valid(ln: &LineMatch) -> usize {
    todo!()
}

fn get_symbols(str: &str) -> LineMatch {
    let re_numbers: Regex = Regex::new(r"(\d+)").unwrap();
    let re_symbols: Regex = Regex::new(r"[^0-9\.\s]").unwrap();

    let numbers: Vec<Match> = re_numbers
        .captures_iter(str)
        .map(|c| c.get(0).unwrap())
        .collect();

    let symbols: Vec<Match> = re_symbols
        .captures_iter(str)
        .map(|c| c.get(0).unwrap())
        .collect();

    LineMatch {
        number_matches: numbers,
        symbol_matches: symbols,
    }
}

struct LineMatch<'a> {
    number_matches: Vec<Match<'a>>,
    symbol_matches: Vec<Match<'a>>,
}
