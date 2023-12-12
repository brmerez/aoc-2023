use regex::{Match, Regex};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let matches: Vec<LineMatch> = file.lines().map(|line| get_symbols(line)).collect();
    let total = matches.iter().enumerate().fold(0, |val, (index, m)| {
        let mut _index = index;

        if index > 0 {
            _index = index - 1;
        }

        let prev = matches.get(_index);
        let next = matches.get(index + 1);
        println!("Linha {index}");

        val + get_sum_of_valid(m, prev, next)
    });
    println!("Total: {total}");
}

fn get_sum_of_valid(ln: &LineMatch, prev: Option<&LineMatch>, next: Option<&LineMatch>) -> usize {
    let mut line_total: usize = 0;

    for num in ln.number_matches.iter() {
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
            line_total += b;
        }
    }

    line_total
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

fn get_symbols(str: &str) -> LineMatch {
    let re_numbers: Regex = Regex::new(r"(\d+)").unwrap();

    let numbers: Vec<Match> = re_numbers
        .captures_iter(str)
        .map(|c| c.get(0).unwrap())
        .collect();

    LineMatch {
        number_matches: numbers,
        full: str.to_owned(),
    }
}

struct LineMatch<'a> {
    number_matches: Vec<Match<'a>>,
    full: String,
}
