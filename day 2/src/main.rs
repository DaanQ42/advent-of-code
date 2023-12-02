use std::{fs, path::Path};

use crate::models::game_stats::{Game, SetCubes};

mod models;

fn main() {
    let inputfile = Path::new("input.txt");
    let contents = fs::read_to_string(inputfile).unwrap();

    let games: Vec<Game> = contents
        .lines()
        .filter_map(|line| Game::parse(line))
        .collect();

    assert!(games.iter().count() == 100);

    let sum: i32 = games
        .iter()
        .filter(|g| what_the_elf_wants(g))
        .map(|g| g.id)
        .sum();

    println!("sum is: {}", sum);

    //Part II
    let sum: i32 = games.iter().map(|g| power(&g)).sum();

    println!("sum is: {}", sum);
}

fn what_the_elf_wants(game: &Game) -> bool {
    // If only 12 red cubes, 13 green cubes, and 14 blue cubes was possible
    let not_higher_than_any = SetCubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let possible = game.cubes.iter().all(|m| {
        m.blue <= not_higher_than_any.blue
            && m.green <= not_higher_than_any.green
            && m.red <= not_higher_than_any.red
    });

    return possible;
}

fn power(game: &Game) -> i32 {
    // Find the minimum amount of cubes needed
    let mut most_red = i32::MIN;
    let mut most_green = i32::MIN;
    let mut most_blue = i32::MIN;

    for c in game.cubes.iter() {
        most_red = most_red.max(c.red);
        most_green = most_green.max(c.green);
        most_blue = most_blue.max(c.blue);
    }

    // println!(
    //     "Game {}, red {}, green {}, blue {};",
    //     game.id, most_red, most_green, most_blue
    // );

    let sum = most_red * most_green * most_blue;

    return sum;
}
