use std::fs;

fn main() {
    let file: String = fs::read_to_string("input.txt").unwrap();
    let cards: Vec<Card> = file.lines().map(strip_line).collect();
    let total = cards.iter().fold(0, |acc, card| {
        let points = check_winners(card);
        acc + points
    });

    println!("Total de pontos: {total}");
}

fn check_winners(card: &Card) -> i32 {
    let points = card.played_numbers.iter().fold(0, |acc, played_num| {
        // Provavelmente há um jeito de fazer isso mais rápido
        // fazendo parse dos números, dando sort no vetor e usando binary search
        if card.winning_numbers.contains(played_num) {
            if acc == 0 {
                return acc + 1;
            }
            return acc * 2;
        }
        acc
    });

    points
}

fn strip_line(line: &str) -> Card {
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

    Card {
        winning_numbers,
        played_numbers,
    }
}

struct Card {
    winning_numbers: Vec<String>,
    played_numbers: Vec<String>,
}
