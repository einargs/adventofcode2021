use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

struct Position {
    x: u32,
    y: u32,
}

fn move_submarine(inp: Vec<String>) -> u32 {
    // start at (0, 0)
    let mut pos = Position { x: 0, y: 0 };
    for dir in inp.into_iter() {
        let mut it = dir.split_whitespace();
        let direction = it.next().unwrap();
        let amt = it.next().unwrap().to_owned().parse::<u32>().unwrap();
        match direction {
            "down" => pos.y += amt,
            "up" => pos.y -= amt,
            "forward" => pos.x += amt,
            _ => panic!("Invalid input direction."),
        };
    }
    pos.x * pos.y
}

fn move_submarine_2(inp: Vec<String>) -> u32 {
    // start at (0, 0)
    let mut pos = Position { x: 0, y: 0 };
    let mut aim = 0;
    for dir in inp.into_iter() {
        let mut it = dir.split_whitespace();
        let direction = it.next().unwrap();
        let amt = it.next().unwrap().to_owned().parse::<u32>().unwrap();
        match direction {
            "down" => aim += amt,
            "up" => aim -= amt,
            "forward" => {
                pos.x += amt;
                pos.y += amt * aim;
            }
            _ => panic!("Invalid input direction."),
        };
    }
    pos.x * pos.y
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let instructions: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_owned())
        .collect();

    Ok(move_submarine(instructions).to_string())
}
pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let instructions: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_owned())
        .collect();

    Ok(move_submarine_2(instructions).to_string())
}

#[test]
fn test_part_1() {
    assert_eq!(
        move_submarine(
            vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .into_iter()
            .map(|x| x.to_owned())
            .collect()
        ),
        150
    );
}

#[test]
fn test_part_2() {
    assert_eq!(
        move_submarine_2(
            vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .into_iter()
            .map(|x| x.to_owned())
            .collect()
        ),
        900
    );
}
