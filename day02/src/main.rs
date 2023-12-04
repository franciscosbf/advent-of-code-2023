use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;

    // NOTE: only 12 red cubes, 13 green cubes, and 14 blue cubes

    input.iter().for_each(|line| {
        let splitted: Vec<&str> = line.split(':').collect();

        for raw_sets in splitted.get(1).unwrap().split(';') {
            for raw_hand in raw_sets.trim().split(", ") {
                let mut raw_pair = raw_hand.split(' ');
                let count = raw_pair
                    .next().unwrap()
                    .parse::<u32>().unwrap();
                let max = match raw_pair.next().unwrap() {
                    "red" => 12, "green" => 13, "blue" => 14,  _ => 0,
                };
                if count > max { return; }
            }
        }

        let game = splitted
            .first().unwrap()
            .split(' ').nth(1).unwrap()
            .parse::<u32>().unwrap();
        sum += game;
    });

    println!("{sum}");

    Ok(())
}
