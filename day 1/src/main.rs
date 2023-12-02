use std::{fs, path::Path};

fn main() {
    let inputfile = Path::new("input.txt");
    let contents = fs::read_to_string(inputfile).unwrap();

    let mut sum: u64 = 0;

    for line in contents.lines() {
        let value = get_calibration_value(line);
        sum += value;
        //println!("  {} from {}", value, line);
    }

    println!("sum is: {}", sum);
}

const ZERO_OFFSET: u64 = '0' as u64;

fn get_calibration_value(data: &str) -> u64 {
    // Check if there is any item inside
    let values = (0..data.len()).filter_map(|i| value_at(data, i));

    // Any values?
    if values.clone().peekable().peek().is_none() {
        return 0;
    }

    let first = values.clone().nth(0).unwrap();
    let second = values.last().unwrap();

    return first * 10 + second;
}

fn value_at(data: &str, index: usize) -> Option<u64> {
    let c = data.chars().nth(index).unwrap_or('\0');
    if c.is_numeric() {
        return Some((c as u64) - ZERO_OFFSET);
    }

    if !c.is_ascii_lowercase() {
        return None;
    }

    //grouped by word length

    // 3 length
    // one, two, six
    let range = index..(index + 3);
    match data.get(range) {
        Some("one") => return Some(1),
        Some("two") => return Some(2),
        Some("six") => return Some(6),
        _ => {}
    };

    // 4 length
    // zero, four, five, nine
    let range = index..(index + 4);
    match data.get(range) {
        Some("zero") => return Some(0),
        Some("four") => return Some(4),
        Some("five") => return Some(5),
        Some("nine") => return Some(9),
        _ => {}
    };

    // 5 length
    // three, seven, eight
    let range = index..(index + 5);
    match data.get(range) {
        Some("three") => return Some(3),
        Some("seven") => return Some(7),
        Some("eight") => return Some(8),
        _ => {}
    };

    None
}
