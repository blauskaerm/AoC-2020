use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calc_range(boarding_directions: &Vec<char>,
	      high_seat_range: usize,
	      low_seat_range: usize,
	      high_range_char: char,
	      low_range_char: char,
	      high_range: usize,
	      low_range: usize) -> usize {

    let mut high_r = high_seat_range;
    let mut low_r = low_seat_range;
    
    for i in low_range..high_range {
	println!("{}: {}-{} ({})", boarding_directions[i], low_r, high_r, high_r - low_r);
	//println!("{}", boarding_directions[i]);
	if boarding_directions[i] == high_range_char {
	    high_r -= (high_r - low_r) / 2;
	} else if boarding_directions[i] == low_range_char {
	    low_r += (high_r - low_r) / 2;
	}
	
    }
    //print!("Select: {}: ", boarding_directions[high_range - 1]);
    let result = if boarding_directions[high_range - 1] == high_range_char {
	//println!("Low");
	low_r
    } else {
	//println!("High");
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
    let file = File::open("input2")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
	let line = line.unwrap();
	let bording_instructions: Vec<char> = line.chars().collect();

	let (row, col, checksum) = calc_boarding_seat(&bording_instructions);
	
	println!("{} row: {}, column: {} ({})", line, row, col, checksum);
	println!();
    }

    // let mut low_col_range = 0;
    // let mut high_col_range = 127;

    

    // let direction: Vec<char> = "BBFFBBFRLL".chars().collect();
    // for i in 0..7 {
    // 	if direction[i] == 'F' {
    // 	    high_col_range -= ((high_col_range - low_col_range) / 2) -1;
    // 	} else if direction[i] == 'B' {
    // 	    low_col_range += (high_col_range - low_col_range) / 2;
    // 	}
    // 	println!("{}-{}", low_col_range, high_col_range);
    // }
    // let final_row = if direction[6] == 'F' {
    // 	low_col_range
    // } else {
    // 	high_col_range
    // };

    // println!("Final row: {}", final_row);

    // let mut low_row_range = 0;
    // let mut high_row_range = 8;
    

    
    Ok(())
}
