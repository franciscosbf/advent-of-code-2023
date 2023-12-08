use std::{fs, error::Error};

fn is_adjacent(s: &[u8], i: usize) -> bool {
    !s[i].is_ascii_digit() && s[i] != b'.'
}

// expects the full line in each slice and global positions.
fn is_part_number_up(
    la: &[u8], cl: &[u8],
    gs: usize, ge: usize,
) -> bool {
    let s = if gs > 0 {
        let _s = gs-1;
        if is_adjacent(cl, _s) {
            return true;
        }
        _s
    } else { 0 };

    let last = cl.len()-1;
    let e = (ge+1).min(last);
    if e < last && is_adjacent(cl, last) {
        return true;
    }

    (s..=e).any(|i| is_adjacent(la, i))
}

// expects the full line in each slice and global positions.
fn is_part_number_middle(
    la: &[u8], cl: &[u8], lb: &[u8],
    gs: usize, ge: usize,
) -> bool {
    let s = if gs > 0 {
        let _s = gs-1;
        if is_adjacent(cl, _s) {
            return true;
        }
        _s
    } else { 0 };

    let last = cl.len()-1;
    let e = (ge+1).min(last);
    if e < last && is_adjacent(cl, e) {
        return true;
    }

    (s..=e).any(|i| is_adjacent(la, i) || is_adjacent(lb, i))
}

// expects the full line in each slice and global positions.
fn is_part_number_down(
    cl: &[u8], lb: &[u8],
    gs: usize, ge: usize,
) -> bool {
    let s = if gs > 0 {
        let _s = gs-1;
        if is_adjacent(cl, _s) {
            return true;
        }
        _s
    } else { 0 };

    let last = cl.len()-1;
    let e = (ge+1).min(last);
    if e < last && is_adjacent(cl, last) {
        return true;
    }

    (s..=e).any(|i| is_adjacent(lb, i))
}

// returned positions are relative to the given line_slice.
// The given slice might be a substring of line.
//
// NOTE: assumes that slice isn't empty.
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

fn convert(s: &[u8]) -> u32 {
    let mut n = 0;
    s.iter().for_each(|&d| n = (n * 10) + ((d - b'0') as u32));
    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let input: Vec<&[u8]> = file
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let mut sum: u32 = 0;

    // WARN: assumes that there's at least 3 lines.

    let curr = *input.get(0).unwrap();
    let down = *input.get(1).unwrap();
    let mut last_end = 0;
    loop {
        let (more, s, e) = search_first_number(&curr[last_end..]);
        if !more { break; }
        let gs = s + last_end;
        let mut ge = e + last_end;
        if is_part_number_down(curr, down, gs, ge) {
            sum += convert(&curr[gs..=ge]);
        }
        ge += 2;
        if ge >= curr.len() { break; }
        last_end = ge;
    }

    for i in 1..input.len()-1 {
        let up = *input.get(i-1).unwrap();
        let curr = *input.get(i).unwrap();
        let down = *input.get(i+1).unwrap();
        let mut last_end = 0;
        loop {
            let (more, s, e) = search_first_number(&curr[last_end..]);
            if !more { break; }
            let gs = s + last_end;
            let mut ge = e + last_end;
            if is_part_number_middle(up, curr, down, gs, ge) {
                sum += convert(&curr[gs..=ge]);
            }
            ge += 2;
            if ge >= curr.len() { break; }
            last_end = ge;
        }
    }

    let up = *input.get(input.len() - 2).unwrap();
    let curr = *input.last().unwrap();
    let mut last_end = 0;
    loop {
        let (more, s, e) = search_first_number(&curr[last_end..]);
        if !more { break; }
        let gs = s + last_end;
        let mut ge = e + last_end;
        if is_part_number_up(up, curr, gs, ge) {
            sum += convert(&curr[gs..=ge]);
        }
        ge += 2;
        if ge >= curr.len() { break; }
        last_end = ge;
    }

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

    #[test]
    fn test_is_part_number_middle() {
        let sa = ".......".as_bytes();
        let sm = "..123..".as_bytes();
        let sb = ".......".as_bytes();

        assert!(!is_part_number_middle(sa, sm, sb, 2, 4), "normal");

        let sa = ".......".as_bytes();
        let sm = ".#123..".as_bytes();
        let sb = ".......".as_bytes();

        assert!(is_part_number_middle(sa, sm, sb, 2, 4), "left symbol in the same line");

        let sa = ".......".as_bytes();
        let sm = "..123&.".as_bytes();
        let sb = ".......".as_bytes();

        assert!(is_part_number_middle(sa, sm, sb, 2, 4), "right symbol in the line");

        let sa = ".$.....".as_bytes();
        let sm = "..123..".as_bytes();
        let sb = ".......".as_bytes();

        assert!(is_part_number_middle(sa, sm, sb, 2, 4), "left diagonal in line above");

        let sa = ".......".as_bytes();
        let sm = "..123..".as_bytes();
        let sb = ".....%.".as_bytes();

        assert!(is_part_number_middle(sa, sm, sb, 2, 4), "right diagonal in line bellow");

        let sa = ".......".as_bytes();
        let sm = "123....".as_bytes();
        let sb = ".......".as_bytes();

        assert!(!is_part_number_middle(sa, sm, sb, 0, 2), "normal at the beginning of the line");

        let sa = ".......".as_bytes();
        let sm = "....123".as_bytes();
        let sb = ".......".as_bytes();

        assert!(!is_part_number_middle(sa, sm, sb, 0, 2), "normal in the end of the line");
    }
}
