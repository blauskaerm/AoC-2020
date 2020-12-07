use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut answer_index = 1;
    let mut answer_vec: Vec<char> = Vec::new();
    let mut answer_summarize_vec: Vec<usize> = Vec::with_capacity(1000);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars: Vec<char> = line.chars().collect();

        answer_vec.append(&mut chars);
        answer_vec.sort();
        answer_vec.dedup();

        if answer_summarize_vec.len() < answer_index {
            answer_summarize_vec.push(answer_vec.len());
        } else {
            answer_summarize_vec[answer_index - 1] = answer_vec.len();
        }

        if line.len() == 0 {
            answer_vec.clear();
            answer_index += 1;
        }
    }

    let mut total_answer_amount = 0;
    for answer in &answer_summarize_vec {
        total_answer_amount += answer;
    }

    println!("Answers: {}", total_answer_amount);

    Ok(())
}
