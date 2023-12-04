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
        let mut l: u32 = 0;
        for chr in line.chars() {
            // check digit
            if chr.is_numeric() {
                l = chr.to_digit(10).unwrap();
                break;
            }
        }
        let mut r: u32 = 0; // assumes that size is == u32.
        for chr in line.chars().rev() {
            // check digit
            if chr.is_numeric() {
                r = chr.to_digit(10).unwrap();
                break;
            }
        }
        let value = (l * 10) + r;
        sum += value;
    });

    println!("{sum}");

    Ok(())
}
