use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum PointType {
    Land,
    Tree,
}

impl fmt::Display for PointType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let point_display = match self {
            PointType::Land => '.',
            PointType::Tree => '#',
        };
        write!(f, "{}", point_display)
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<PointType>> = Vec::new();

    for line in reader.lines() {
        let mut map_row: Vec<PointType> = Vec::new();
        for point in line?.chars() {
            map_row.push(match point {
                '.' => PointType::Land,
                '#' => PointType::Tree,
                _ => PointType::Land,
            });
        }
        map.push(map_row);
    }

    let mut x = 0;
    let mut y = 0;
    let mut nr_of_trees_hit = 0;
    for map_row in &map {
        if map_row[x] == PointType::Tree {
            nr_of_trees_hit += 1;
        }
        x += 3;
        x %= 31;
        y += 1;
    }

    println!("Nr of trees hit: {}", nr_of_trees_hit);

    Ok(())
}
