use std::{fs, error::Error, primitive};

// expects the full line in each slice and global positions.
fn is_part_number_up(
    line_above: &str, line: &str,
    global_start: usize, global_end: usize,
) -> bool {
    todo!()
}

// expects the full line in each slice and global positions.
fn is_part_number_middle(
    line_above: &str, line: &str, line_below: &str,
    global_start: usize, global_end: usize,
) -> bool {
    todo!()
}

// expects the full line in each slice and global positions.
fn is_part_number_down(
    line_below: &str, line: &str,
    global_start: usize, global_end: usize,
) -> bool {
    todo!()
    // if global_start > 0 && line.get(global_start - 1).unwrap() != '.' { return true; }
}

// returned positions are relative to the given line_slice.
// The given slice might be a substring of line.
fn search_first_number(slice: &[u8]) -> (bool, usize, usize) {
    if slice.is_empty() { return (false, 0, 0); }

    let mut line_slice = slice.iter();
    let mut relative_start = 0;

    for &chr in &mut line_slice {
        if chr.is_ascii_digit() { break; }
        relative_start += 1;
    }

    // for loop didn't find a digit.
    if relative_start == slice.len() { return (false, 0, 0); }

    let mut relative_end = relative_start;

    for &chr in &mut line_slice {
        if !chr.is_ascii_digit() { break; }
        relative_end += 1;
    }

    (true, relative_start, relative_end)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let input: Vec<&[u8]> = file
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let mut sum: u32 = 0;

    // TODO: check first line.

    for i in 1..input.len()-1 {
        // TODO: check inner lines.
    }

    // TODO: check last line.

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_first_number_with_multiple_digits_number() {
        let s = ".....114.123..".as_bytes();

        let (found, start, end) = search_first_number(s);

        assert!(found);
        assert_eq!(start, 5);
        assert_eq!(end, 7);
    }

    #[test]
    fn test_search_first_number_with_one_digit() {
        let s = "..1.4.".as_bytes();
        let (found, start, end) = search_first_number(s);

        assert!(found);
        assert_eq!(start, 2);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_search_first_number_with_empty_slice() {
        let s = "".as_bytes();

        let (found, start, end) = search_first_number(s);

        assert!(!found);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }

    #[test]
    fn test_search_first_number_without_numbers() {
        let s = ".....".as_bytes();

        let (found, start, end) = search_first_number(s);

        assert!(!found);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }

    #[test]
    fn test_search_first_number_at_the_start() {
        let s = "123....".as_bytes();

        let (found, start, end) = search_first_number(s);

        assert!(found);
        assert_eq!(start, 0);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_search_first_number_in_the_end() {
        let s = "....123".as_bytes();

        let (found, start, end) = search_first_number(s);

        assert!(found);
        assert_eq!(start, 4);
        assert_eq!(end, 6);
    }
}
