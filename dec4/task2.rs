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

fn check_year_range(value: &str, low: u32, heigh: u32) -> bool {
    let value_num = value.parse::<u32>().unwrap_or(0);
    value.len() == 4 && (low <= value_num && value_num <= heigh)
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
            let value = passport_component_s[1];

            match key {
                "byr" => {
                    passport.birth_year = check_year_range(&value, 1920, 2002);
                }
                "iyr" => {
                    passport.issue_year = check_year_range(&value, 2010, 2020);
                }
                "eyr" => {
                    passport.expiration_year = check_year_range(&value, 2020, 2030);
                }
                "hgt" => {
                    let length = value.len();
                    let unit = &value[length - 2..length];
                    let heigth = &value[0..length - 2].parse::<u32>().unwrap_or(0);

                    passport.height = if unit == "cm" {
                        &150 <= heigth && heigth <= &193
                    } else if unit == "in" {
                        &59 <= heigth && heigth <= &76
                    } else {
                        false
                    };
                }
                "hcl" => {
                    if value.len() == 7 && &value[0..1] == "#" {
                        let mut hair_color_valid = true;
                        for c in value[1..].chars() {
                            let char_valid = ('0' <= c && c <= '9') || ('a' <= c && c <= 'f');
                            hair_color_valid &= char_valid;
                        }

                        passport.hair_color = hair_color_valid;
                    } else {
                        passport.hair_color = false;
                    }
                }
                "ecl" => {
                    passport.eye_color = match value {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false,
                    };
                }
                "pid" => {
                    if value.len() == 9 {
                        let mut passport_id_valid = true;
                        for c in value.chars() {
                            let char_valid = '0' <= c && c <= '9';
                            passport_id_valid &= char_valid;
                        }
                        passport.passport_id = passport_id_valid;
                    } else {
                        passport.passport_id = false;
                    }
                }
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
