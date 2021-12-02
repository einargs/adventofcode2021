use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};

fn count_incs(nums: Vec<i32>) -> u32 {
    nums.windows(2).fold(
        0,
        |acc, slice| if slice[1] > slice[0] { acc + 1 } else { acc },
    )
}

fn count_window_incs(nums: Vec<i32>) -> u32 {
    nums.windows(4).fold(0, |acc, slices| {
        let a = slices[1] + slices[2] + slices[3];
        let b = slices[0] + slices[1] + slices[2];
        if a > b {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let nums: Vec<i32> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(count_incs(nums).to_string())
}
pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let nums: Vec<i32> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(count_window_incs(nums).to_string())
}

#[test]
fn test_part_1() {
    assert_eq!(
        count_incs(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7
    );
}

#[test]
fn test_part_2() {
    assert_eq!(
        count_incs(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        5
    );
}
