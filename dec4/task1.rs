use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Passport {
    birth_year: bool,
    issue_year: bool,
    expiration_year: bool,
    height: bool,
    hair_color: bool,
    eye_color: bool,
    passport_id: bool,
    country_id: bool,
}

impl Passport {
    pub fn passport_is_valid(&self) -> bool {
        self.birth_year
            && self.issue_year
            && self.expiration_year
            && self.height
            && self.hair_color
            && self.eye_color
            && self.passport_id
    }

    pub fn clear_data(&mut self) {
        self.birth_year = false;
        self.issue_year = false;
        self.expiration_year = false;
        self.height = false;
        self.hair_color = false;
        self.eye_color = false;
        self.passport_id = false;
        self.country_id = false;
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut valid_passports = 0;

    let mut passport = Passport {
        birth_year: false,
        issue_year: false,
        expiration_year: false,
        height: false,
        hair_color: false,
        eye_color: false,
        passport_id: false,
        country_id: false,
    };

    for line in reader.lines() {
        let log_line = line.unwrap();

        let passport_components = log_line.split_whitespace();

        for passport_component in passport_components {
            let passport_component_s: Vec<&str> = passport_component.split(":").collect();
            let key = passport_component_s[0];
            let _value = passport_component_s[1];

            match key {
                "byr" => passport.birth_year = true,
                "iyr" => passport.issue_year = true,
                "eyr" => passport.expiration_year = true,
                "hgt" => passport.height = true,
                "hcl" => passport.hair_color = true,
                "ecl" => passport.eye_color = true,
                "pid" => passport.passport_id = true,
                "cid" => passport.country_id = true,
                &_ => println!("?????"),
            }
        }

        if passport.passport_is_valid() {
            valid_passports += 1;
            passport.clear_data();
        }

        if log_line.len() == 0 {
            passport.clear_data();
        }
    }

    println!("Valid passports: {}", valid_passports);

    Ok(())
}
