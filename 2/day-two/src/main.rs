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
    let games: Vec<Vec<Vec<Pair>>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| parse_line(x.to_lowercase().as_str()))
        .collect();

    let mut minimum_colors_exp: usize = 0;
    let mut correct_games_sum: usize = 0;

    for (i, game) in games.iter().enumerate() {
        println!("Game {}", i + 1);

        // Part one of the challenge
        if check_game_valid(&game) {
            correct_games_sum += (i + 1);
        }

        // Part two
        // finding the minimum number of each color
        minimum_colors_exp += add_minimuns(&game);
    }

    println!("Sum of the ids of plausible games: {}", correct_games_sum);
    println!(
        "Sum of the powers of the min number of balls: {}",
        minimum_colors_exp
    );
}

fn add_minimuns(game: &Vec<Vec<Pair>>) -> usize {
    let mut min_r = 0;
    let mut min_b = 0;
    let mut min_g = 0;

    for set in game {
        for pair in set {
            match pair.color {
                Colors::Red => {
                    if pair.value > min_r {
                        min_r = pair.value;
                    }
                }
                Colors::Green => {
                    if pair.value > min_g {
                        min_g = pair.value;
                    }
                }
                Colors::Blue => {
                    if pair.value > min_b {
                        min_b = pair.value;
                    }
                }
            }
        }
    }

    (min_b * min_g * min_r)
}

fn check_game_valid(game: &Vec<Vec<Pair>>) -> bool {
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

    game_valid
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
