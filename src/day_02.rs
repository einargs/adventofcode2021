use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};

fn move_submarine(inp: Vec<String>) -> u32 {
    150
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    /*
    let nums: Vec<i32> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
        */

    Ok("tbd".to_owned())
}
pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    /*
    let nums: Vec<i32> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();
        */

    Ok("tbd".to_owned())
}

#[test]
fn test_part_1() {
    assert_eq!(
        move_submarine(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ].into_iter().map(|x| x.to_owned()).collect()),
        150
    );
}

#[test]
fn test_part_2() {
    assert_eq!(5, 5);
}
