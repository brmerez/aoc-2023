use std::fs;

fn main() {
    let file: String = fs::read_to_string("input.txt").unwrap();
    let cards: Vec<Card> = file.lines().map(parse_line).collect();
    let total = cards.iter().fold(0, |acc, card| acc + check_winners(card));

    println!("Total de pontos: {total}");
}

fn check_winners(card: &Card) -> i32 {
    let points = card.played_numbers.iter().fold(0, |acc, played_num| {
        // Provavelmente há um jeito de fazer isso mais rápido
        // fazendo parse dos números, dando sort no vetor e usando binary search
        if !card.winning_numbers.contains(played_num) {
            return acc;
        }

        return if acc == 0 { acc + 1 } else { acc * 2 };
    });

    return points;
}

fn parse_line(line: &str) -> Card {
    let mut result = line.split(" | ");
    let winners = result.next().unwrap();
    let played = result.next().unwrap();

    let winning_numbers: Vec<String> = winners
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().to_owned())
        .collect();

    let played_numbers: Vec<String> = played
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().to_owned())
        .collect();

    return Card {
        winning_numbers,
        played_numbers,
    };
}

struct Card {
    winning_numbers: Vec<String>,
    played_numbers: Vec<String>,
}
