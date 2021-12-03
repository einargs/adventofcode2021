use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use std::cmp::{Ord, Ordering};

fn parse(filename: &str) -> Result<(usize, Vec<u32>), io::Error> {
    let f = File::open(filename)?;

    let mut reader = BufReader::new(&f);
    let mut iter = reader.by_ref().lines();
    let firstStr = iter.next().unwrap()?;
    let len = firstStr.len();
    let firstNum = u32::from_str_radix(&firstStr, 2).unwrap();
    let nums: Vec<u32> = std::iter::once(firstNum)
        .chain(iter.map(|l| u32::from_str_radix(&l.unwrap(), 2).unwrap()))
        .collect();
    Ok((len, nums))
}

fn bit_at(num: u32, len: usize, idx: usize) -> bool {
    ((num >> (len - idx - 1)) & 1) == 1
}

fn cmp_1_to_0(nums: &[u32], len: usize, idx: usize) -> Ordering {
    let indicator: i32 = nums.iter().fold(0, |count, v|
        if bit_at(*v, len, idx) { count + 1 } else { count - 1}
    );
    indicator.cmp(&0)
}

fn parse_diagnostics(len: usize, nums: Vec<u32>) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..len {
        match cmp_1_to_0(&nums, len, i) {
            Ordering::Greater => {
                gamma = (gamma << 1) | 1;
                epsilon = epsilon << 1;
            }
            Ordering::Less => {
                epsilon = (epsilon << 1) | 1;
                gamma = gamma << 1;
            }
            Ordering::Equal => panic!("at index {:?}", i),
        }
    }
    gamma * epsilon
}

fn apply_bit_criteria<F>(len: usize, inp: Vec<u32>, crit: F) -> u32
where
    F: Fn(Ordering) -> bool,
{
    let mut nums = inp;
    for i in 0..len {
        let desiredBit = crit(cmp_1_to_0(&nums, len, i));
        nums = nums
            .into_iter()
            .filter(|v| bit_at(*v, len, i) == desiredBit)
            .collect();
        if nums.len() == 1 {
            return nums[0];
        }
    }
    panic!("Should never reach! Should be narrowed down to one.")
}

fn parse_life_support_rating(len: usize, nums: Vec<u32>) -> u32 {
    let oxygen = apply_bit_criteria(len, nums.clone(), |o| o != Ordering::Less);
    let co2 = apply_bit_criteria(len, nums, |o| o == Ordering::Less);
    oxygen * co2
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let (len, diagnostics) = parse(filename)?;

    Ok(parse_diagnostics(len, diagnostics).to_string())
}
pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let (len, diagnostics) = parse(filename)?;

    Ok(parse_life_support_rating(len, diagnostics).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<u32> {
        vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ]
    }

    #[test]
    fn test_bit_at() {
        assert_eq!(bit_at(0b0100, 4, 0), false);
        assert_eq!(bit_at(0b0100, 4, 1), true);
        assert_eq!(bit_at(0b1011, 4, 1), false);
    }

    #[test]
    fn test_cmp_1_to_0() {
        let nums = test_data();
        assert_eq!(cmp_1_to_0(&nums, 5, 0), Ordering::Greater);
        assert_eq!(cmp_1_to_0(&nums, 5, 1), Ordering::Less);
        
        let nums2 = vec![ 30, 22, 23, 21, 31, 28, 16, 25 ];
        assert_eq!(cmp_1_to_0(&nums2, 5, 1), Ordering::Equal);
    }

    #[test]
    fn test_part_1() {
        let nums = test_data();
        assert_eq!(parse_diagnostics(5, nums), 198);
    }

    #[test]
    fn test_part_2() {
        let nums = test_data();
        assert_eq!(parse_life_support_rating(5, nums), 230);
    }
}
