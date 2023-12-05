use std::fmt::{Debug, Formatter, Result};
use std::fs;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

struct Pair {
    value: usize,
    color: Colors,
}

impl Debug for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {:?})", self.value, self.color)
    }
}

impl Pair {
    fn from_tuple(tuple: (usize, String)) -> Pair {
        let color = match tuple.1.as_str() {
            "red" => Colors::Red,
            "green" => Colors::Green,
            "blue" => Colors::Blue,
            _ => panic!("Invalid color: {}", tuple.1),
        };

        Pair {
            value: tuple.0,
            color: color,
        }
    }
}

fn main() {
    const MAX_R: Pair = Pair {
        value: 12,
        color: Colors::Red,
    };
    const MAX_G: Pair = Pair {
        value: 13,
        color: Colors::Green,
    };
    const MAX_B: Pair = Pair {
        value: 14,
        color: Colors::Blue,
    };

    let games: Vec<Vec<Vec<Pair>>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| parse_line(x.to_lowercase().as_str()))
        .collect();

    let mut total: usize = 0;

    for (i, game) in games.iter().enumerate() {
        println!("Game {}", i + 1);

        let mut game_valid: bool = true;
        for set in game {
            for pair in set {
                let valid: bool = match pair.color {
                    Colors::Red => pair.value <= MAX_R.value,
                    Colors::Green => pair.value <= MAX_G.value,
                    Colors::Blue => pair.value <= MAX_B.value,
                };

                if !valid {
                    game_valid = false;
                    break;
                }
            }
        }

        if game_valid {
            total += (i + 1);
        }
    }

    println!("Total: {}", total);
}

fn parse_line(line: &str) -> Vec<Vec<Pair>> {
    let b = line.split(";");
    let mut results: Vec<Vec<Pair>> = Vec::new();

    for r in b {
        let split: Vec<&str> = r.split(",").collect::<Vec<&str>>();
        let games: Vec<Pair> = split
            .iter()
            .map(|x: &&str| {
                let b: Vec<&str> = x.trim().split(" ").collect::<Vec<&str>>();
                let value: usize = b[0].parse::<usize>().unwrap();
                let color: String = b[1].to_string().replace("[\\W]", "");
                Pair::from_tuple((value, color))
            })
            .collect();

        results.push(games);
    }

    results
}
