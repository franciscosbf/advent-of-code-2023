use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;

    input.iter().for_each(|line| {
        let splitted: Vec<&str> = line.split(':').collect();

        let mut red = 0; let mut green = 0; let mut blue = 0;

        for raw_sets in splitted.get(1).unwrap().split(';') {
            for raw_hand in raw_sets.trim().split(", ") {
                let mut raw_pair = raw_hand.split(' ');
                let count = raw_pair
                    .next().unwrap()
                    .parse::<u32>().unwrap();
                match raw_pair.next().unwrap() {
                    "red" => if count > red { red = count; },
                    "green" => if count > green { green = count; },
                    "blue" => if count > blue { blue = count; },
                    _ => (),
                };
            }
        }

        sum += red * green * blue;
    });

    println!("{sum}");

    Ok(())
}
