use regex::{Match, Regex};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let numeros: Vec<LineMatch> = file.lines().map(|line| get_numbers(line)).collect();
    let total = numeros.iter().enumerate().fold(0, |val, (index, m)| {
        let mut _index = index;

        if index > 0 {
            _index = index - 1;
        }

        let prev = numeros.get(_index);
        let next = numeros.get(index + 1);
        println!("Linha {index}");

        val + get_sum_of_valid(m, prev, next)
    });
    println!("Total: {total}");
}

fn get_sum_of_valid(ln: &LineMatch, prev: Option<&LineMatch>, next: Option<&LineMatch>) -> usize {
    let total = ln.symbol_matches.iter().fold(0, |acc, num| {
        let b: usize = num.as_str().parse().unwrap();
        let mut valid = false;

        // Verificar a linha superior
        if prev.is_some_and(|prev| check_adjacency(num, &prev.full)) {
            valid = true;
        }
        // Verificar as posições adjacentes
        if check_adjacency(num, &ln.full) {
            valid = true;
        }
        // Verificar a linha inferior
        if next.is_some_and(|next| check_adjacency(num, &next.full)) {
            valid = true;
        }

        if valid {
            return acc + b;
        }

        acc
    });

    total
}

fn check_adjacency(m: &Match, line: &str) -> bool {
    let re_symbols: Regex = Regex::new(r"[^0-9\.\s]").unwrap();

    let ini = m.start();
    let end = m.end();
    let line_len = line.len();

    let search_start = if ini > 0 { ini - 1 } else { ini };
    let search_end = if end < line_len { end + 1 } else { end };

    // Check adjacent
    let b = &line[search_start..search_end];
    println!("{b}");

    re_symbols.is_match(b)
}

fn get_numbers(str: &str) -> LineMatch {
    let re_numbers: Regex = Regex::new(r"(\d+)").unwrap();

    let numbers: Vec<Match> = re_numbers
        .captures_iter(str)
        .map(|c| c.get(0).unwrap())
        .collect();

    LineMatch {
        symbol_matches: numbers,
        full: str.to_owned(),
    }
}

struct LineMatch<'a> {
    symbol_matches: Vec<Match<'a>>,
    full: String,
}
