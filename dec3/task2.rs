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

fn check_slope(map: &Vec<Vec<PointType>>, inc_x: usize, inc_y: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut nr_of_trees_hit = 0;
    while y < map.len() {
        if map[y][x] == PointType::Tree {
            nr_of_trees_hit += 1;
        }
        x += inc_x;
        x %= 31;
        y += inc_y;
    }

    nr_of_trees_hit
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

    let nr_of_trees_hit: [usize; 5] = [
        check_slope(&map, 1, 1),
        check_slope(&map, 3, 1),
        check_slope(&map, 5, 1),
        check_slope(&map, 7, 1),
        check_slope(&map, 1, 2),
    ];

    let mut final_hits = 1;
    for nr_of_trees in &nr_of_trees_hit {
        final_hits *= nr_of_trees;
    }

    println!("Nr of trees hit: {}", final_hits);

    Ok(())
}
