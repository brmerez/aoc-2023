use regex::{Match, Regex};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let matches: Vec<LineMatch> = file.lines().map(|line| get_symbols(line)).collect();
    let total = matches.iter().enumerate().fold(0, |val, (index, m)| {
        let index = index + 1;
        let prev = matches.get(index - 1).unwrap();
        let next = matches.get(index + 1).unwrap();

        let r = get_sum_of_valid(m, prev, next);

        val + r
    });
    println!("Total: {total}");
}

fn get_sum_of_valid(ln: &LineMatch, prev: &LineMatch, next: &LineMatch) -> usize {
    // Para cada número:
    //      - Verificar as posições da linha anterior
    //      - Verificar as posições adjacentes
    //      - Verificar as posições da próxima linha (se houver!)

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
