use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Credential {
    bound_l: u32,
    bound_h: u32,
    password: String,
    checker: char,
}

fn check_password(credential: &Credential) -> bool {
    let low_bound: usize = (credential.bound_l as usize) - 1;
    let high_bound: usize = (credential.bound_h as usize) - 1;

    let low: bool = credential.password.chars()
	.nth(low_bound).unwrap_or('-') == credential.checker;

    let high: bool = credential.password.chars()
	.nth(high_bound).unwrap_or('-') == credential.checker;

    low ^ high
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut correct_passwords = 0;
    for line in reader.lines() {
        // Chop chop string
        let credential_check_line = line.unwrap();
        let first_split: Vec<&str> = credential_check_line.split(":").collect();
        let credential_condition: Vec<&str> = first_split[0].split_whitespace().collect();
        let credential_bound: Vec<&str> = credential_condition[0].split("-").collect();
        let credential_bound_low = credential_bound[0];
        let credential_bound_high = credential_bound[1];
        let credential_char = credential_condition[1];
        let password = first_split[1].trim();

        let credential = Credential {
            bound_l: credential_bound_low.parse::<u32>().unwrap(),
            bound_h: credential_bound_high.parse::<u32>().unwrap(),
            password: String::from(password),
            checker: credential_char.chars().nth(0).unwrap(),
        };

        if check_password(&credential) {
            correct_passwords += 1;
        }
    }

    println!("Correct passwords: {}", correct_passwords);

    Ok(())
}
