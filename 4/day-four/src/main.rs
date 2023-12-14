use std::fs;

fn main() {
    let file: String = fs::read_to_string("input.txt").unwrap();
    let cards: Vec<Card> = file.lines().map(parse_line).collect();
    let total = cards.iter().fold(0, |acc, card| acc + check_winners(card));

    println!("Total de pontos: {total}");
}

fn check_winners(card: &Card) -> i32 {
    let points = card.played_numbers.iter().fold(0, |acc, played_num| {
        if card.winning_numbers.binary_search(played_num).is_err() {
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

    let mut winning_numbers: Vec<i32> = winners
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.parse().unwrap())
        .collect();

    let mut played_numbers: Vec<i32> = played
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.parse().unwrap())
        .collect();

    played_numbers.sort();
    winning_numbers.sort();

    return Card {
        winning_numbers,
        played_numbers,
    };
}

struct Card {
    winning_numbers: Vec<i32>,
    played_numbers: Vec<i32>,
}
