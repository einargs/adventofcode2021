use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn to_dec(inp: Vec<u32>) -> u32 {
    inp.iter()
        .rev()
        .enumerate()
        .map(|(i, val)| 2u32.pow(i as u32) * val)
        .sum()
}

fn count_bits(inp: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let zero_counts: Vec<u32> = inp.iter().fold(vec![0; inp[0].len()], |counts, entry| {
        counts
            .iter()
            .zip(entry.chars())
            .map(|(a, b)| if b == '0' { a + 1 } else { *a })
            .collect()
    });
    let one_counts: Vec<u32> = zero_counts.iter().map(|x| inp.len() as u32 - x).collect();
    (zero_counts, one_counts)
}

fn calc_gamma_epsilon_rates(zero_counts: Vec<u32>, one_counts: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let gamma_rate: Vec<u32> = zero_counts
        .iter()
        .zip(one_counts.iter())
        .map(|(z, o)| if z > o { 0 } else { 1 })
        .collect();

    let epsilon_rate: Vec<u32> = gamma_rate.iter().map(|x| 1 - x).collect();

    (gamma_rate, epsilon_rate)
}

fn parse_diagnostics(inp: Vec<String>) -> u32 {
    let (zero_counts, one_counts) = count_bits(inp);
    let (gamma_rate, epsilon_rate) = calc_gamma_epsilon_rates(zero_counts, one_counts);

    to_dec(gamma_rate) * to_dec(epsilon_rate)
}

fn parse_life_support_rating(inp: Vec<String>) -> u32 {
    let mut oxygen = inp.clone();
    let mut carbon = inp.clone();

    let mut i: usize = 0;
    while oxygen.len() > 1 {
        let (zero_counts, one_counts) = count_bits(oxygen.clone());
        let (gamma_rate, _) = calc_gamma_epsilon_rates(zero_counts, one_counts);
        oxygen = oxygen
            .iter()
            .filter(|x| x.get(i..i + 1).unwrap().parse::<u32>().unwrap() == gamma_rate[i])
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        i += 1;
    }

    let oxygen: Vec<u32> = oxygen[0].chars().map(|x| x.to_digit(10).unwrap()).collect();

    let mut i: usize = 0;
    while carbon.len() > 1 {
        let (zero_counts, one_counts) = count_bits(carbon.clone());
        let (_, epsilon_rate) = calc_gamma_epsilon_rates(zero_counts, one_counts);
        carbon = carbon
            .iter()
            .filter(|x| x.get(i..i + 1).unwrap().parse::<u32>().unwrap() == epsilon_rate[i])
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        i += 1;
    }

    let carbon: Vec<u32> = carbon[0].chars().map(|x| x.to_digit(10).unwrap()).collect();

    to_dec(oxygen) * to_dec(carbon)
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let diagnostics: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_owned())
        .collect();

    Ok(parse_diagnostics(diagnostics).to_string())
}
pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let diagnostics: Vec<String> = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| l.unwrap().to_owned())
        .collect();

    Ok(parse_life_support_rating(diagnostics).to_string())
}

#[test]
fn test_part_1() {
    assert_eq!(
        parse_diagnostics(
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .into_iter()
            .map(|x| x.to_owned())
            .collect()
        ),
        198
    );
}

#[test]
fn test_part_2() {
    assert_eq!(
        parse_life_support_rating(
            vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .into_iter()
            .map(|x| x.to_owned())
            .collect()
        ),
        230
    );
}
