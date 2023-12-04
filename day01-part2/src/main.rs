use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;

    /*
    * 3 -> one, two, six
    * 4 -> four, five, nine
    * 5 -> three, seven, eight
    */

    input.iter().for_each(|line| {
        let len = line.len();
        let mut chrs = line.chars();
        let mut l: u32 = 0;
        for i in 0..len {
            // check digit
            let d = chrs.next();
            if d.is_none() { break; }
            let chr = d.unwrap();
            if chr.is_numeric() {
                l = chr.to_digit(10).unwrap();
                break;
            }

            // check digit name
            l = match line.get(i..i+3) {
                Some(name) => {
                    match name {
                        "one" => 1,
                        "two" => 2,
                        "six" => 6,
                        _ => {
                            match line.get(i..i+4) {
                                Some(name) => {
                                    match name {
                                        "four" => 4,
                                        "five" => 5,
                                        "nine" => 9,
                                        _ => {
                                            match line.get(i..i+5) {
                                                Some(name) => {
                                                    match name {
                                                        "three" => 3,
                                                        "seven" => 7,
                                                        "eight" => 8,
                                                        _ => 0,
                                                    }
                                                },
                                                None => 0,
                                            }
                                        },
                                    }
                                },
                                None => 0,
                            }
                        },
                    }
                }
                None => 0,
            };
            if l != 0 { break; }
        }
        let mut chrs = line.chars().rev();
        let mut r: u32 = 0; // assumes that size is == u32.
        for i in 0..len {
            // check digit
            let d = chrs.next();
            if d.is_none() { break; }
            let chr = d.unwrap();
            if chr.is_numeric() {
                r = chr.to_digit(10).unwrap();
                break;
            }

            // check digit name
            let i = len - i;
            if i < 3 { continue; }
            r = match line.get(i-3..i).unwrap() {
                "one" => 1, "two" => 2, "six" => 6,
                _ => {
                    if i < 4 { continue; }
                    match line.get(i-4..i).unwrap() {
                        "four" => 4, "five" => 5, "nine" => 9,
                        _ => {
                            if i < 5 { continue; }
                            match line.get(i-5..i).unwrap() {
                                "three" => 3, "seven" => 7, "eight" => 8,
                                _ => 0,
                            }
                        },
                    }
                },
            };
            if r != 0 { break; }
        }
        let value = (l * 10) + r;
        sum += value;
    });

    println!("{sum}");

    Ok(())
}
