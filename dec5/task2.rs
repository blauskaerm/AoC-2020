use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calc_range(
    boarding_directions: &Vec<char>,
    high_seat_range: usize,
    low_seat_range: usize,
    high_range_char: char,
    low_range_char: char,
    high_range: usize,
    low_range: usize,
) -> usize {
    let mut high_r = high_seat_range;
    let mut low_r = low_seat_range;

    for i in low_range..high_range {
        //println!("{}: {}-{} ({})", boarding_directions[i], low_r, high_r, high_r - low_r);
        if boarding_directions[i] == high_range_char {
            high_r -= ((high_r - low_r) + 1) / 2;
        } else if boarding_directions[i] == low_range_char {
            low_r += ((high_r - low_r) + 1) / 2;
        }
    }
    let result = if boarding_directions[high_range - 1] == high_range_char {
        low_r
    } else {
        high_r
    };

    result
}

fn calc_boarding_seat(bording_instructions: &Vec<char>) -> (usize, usize, usize) {
    let row = calc_range(&bording_instructions, 127, 0, 'F', 'B', 7, 0);
    let col = calc_range(&bording_instructions, 7, 0, 'L', 'R', 10, 7);
    let checksum = 8 * row + col;

    (row, col, checksum)
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut max_boarding_checksum = 0;
    let mut checksum_vec: Vec<usize> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let bording_instructions: Vec<char> = line.chars().collect();

        let (_row, _col, checksum) = calc_boarding_seat(&bording_instructions);

        checksum_vec.push(checksum);
        if checksum > max_boarding_checksum {
            max_boarding_checksum = checksum;
        }
    }

    checksum_vec.sort();
    for i in 0..(checksum_vec.len() - 1) {
        let seat_a = checksum_vec[i];
        let seat_b = checksum_vec[i + 1];

        if seat_a + 1 != seat_b {
            println!("My boarding seat: {}", seat_a + 1);
        }
    }

    Ok(())
}
