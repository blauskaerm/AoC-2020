use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut input_vec: Vec<u32> = Vec::new();
    for line in reader.lines() {
        input_vec.push(line?.parse::<u32>().unwrap());
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 1;
    let input_vec_len = input_vec.len();

    while i < input_vec_len {
        while j < input_vec_len {
            if (input_vec[i] + input_vec[j]) == 2020 {
                println!("Result: {}", input_vec[i] * input_vec[j]);
            }

            j += 1;
        }
        k += 1;
        j = k;
        i += 1;
    }

    Ok(())
}
