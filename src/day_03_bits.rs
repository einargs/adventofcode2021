use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use std::cmp::{Ord, Ordering};

#[derive(Debug,Clone,Copy)]
struct Bits(u32);

impl Bits {
    fn parse(s: &str) -> Bits {
        let len: u32 = s.len().try_into().unwrap();
        let val = u32::from_str_radix(s, 2).unwrap();
        Bits::mk(val, len)
    }

    fn mk(val: u32, len: u32) -> Bits {
        if len >= 32 { panic!("input too large!"); }
        Bits(val | (1 << len))
    }

    fn len(self) -> u32 {
        u32::BITS - 1 - self.0.leading_zeros()
    }

    fn get(self) -> u32 {
        self.0 & !(1 << self.len())
    }

    fn at(self, idx: u32) -> bool {
        ((self.get() >> (self.len() - idx - 1)) & 1) == 1
    }
}

fn parse(filename: &str) -> Result<Vec<Bits>, io::Error> {
    let f = File::open(filename)?;

    let nums: Vec<Bits> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| Bits::parse(&l.unwrap()))
        .collect();
    Ok(nums)
}

fn cmp_1_to_0(nums: &[Bits], idx: u32) -> Ordering {
    let indicator: i32 = nums.iter().fold(0, |count, b|
        if b.at(idx) { count + 1 } else { count - 1}
    );
    indicator.cmp(&0)
}

fn parse_diagnostics(nums: Vec<Bits>) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let f = |r1: &mut u32, r2: &mut u32| {
        *r1 = (*r1 << 1) | 1;
        *r2 = *r2 << 1;
    };
    for i in 0..nums[0].len() {
        match cmp_1_to_0(&nums, i) {
            Ordering::Greater => f(&mut gamma, &mut epsilon),
            Ordering::Less => f(&mut epsilon, &mut gamma),
            Ordering::Equal => panic!("at index {:?}", i),
        }
    }
    gamma * epsilon
}

fn apply_bit_criteria<F>(inp: Vec<Bits>, crit: F) -> Bits
where
    F: Fn(Ordering) -> bool,
{
    let mut nums = inp;
    for i in 0..nums[0].len() {
        let desired_bit = crit(cmp_1_to_0(&nums, i));
        nums = nums
            .into_iter()
            .filter(|b| b.at(i) == desired_bit)
            .collect();
        if nums.len() == 1 {
            return nums[0];
        }
    }
    panic!("Should never reach! Should be narrowed down to one.")
}

fn parse_life_support_rating(nums: Vec<Bits>) -> u32 {
    let oxygen = apply_bit_criteria(nums.clone(), |o| o != Ordering::Less);
    let co2 = apply_bit_criteria(nums, |o| o == Ordering::Less);
    oxygen.get() * co2.get()
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let diagnostics = parse(filename)?;

    Ok(parse_diagnostics(diagnostics).to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let diagnostics = parse(filename)?;

    Ok(parse_life_support_rating(diagnostics).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_bits(len: u32, nums: Vec<u32>) -> Vec<Bits> {
        nums.into_iter().map(|v| Bits::mk(v,len)).collect()
    }

    fn test_data() -> Vec<Bits> {
        to_bits(5, vec![
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
        ])
    }

    #[test]
    fn test_bit_at() {
        assert_eq!(Bits::mk(0b0100, 4).at(0), false);
        assert_eq!(Bits::mk(0b0100, 4).at(1), true);
        assert_eq!(Bits::mk(0b1011, 4).at(1), false);
    }

    #[test]
    fn test_cmp_1_to_0() {
        let nums = test_data();
        assert_eq!(cmp_1_to_0(&nums, 0), Ordering::Greater);
        assert_eq!(cmp_1_to_0(&nums, 1), Ordering::Less);
        
        let nums2 = to_bits(5, vec![ 30, 22, 23, 21, 31, 28, 16, 25 ]);
        assert_eq!(cmp_1_to_0(&nums2, 1), Ordering::Equal);
    }

    #[test]
    fn test_part_1() {
        let nums = test_data();
        assert_eq!(parse_diagnostics(nums), 198);
    }

    #[test]
    fn test_part_2() {
        let nums = test_data();
        assert_eq!(parse_life_support_rating(nums), 230);
    }
}
