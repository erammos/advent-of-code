use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();
        for line in lines {
            let s: Vec<&str> = line.split_whitespace().collect();
            a.push(s[0].parse::<i32>()?);
            b.push(s[1].parse::<i32>()?);
        }
        a.sort();
        b.sort();

        let answer = a
            .iter()
            .zip(b.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum::<i32>();
        Ok(answer as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().flatten();
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();
        let mut i = 0;
        for line in lines {
            let s: Vec<&str> = line.split_whitespace().collect();
            a.push(s[0].parse::<i32>()?);
            b.push(s[1].parse::<i32>()?);
            i += 1;
        }
        a.sort();
        b.sort();

        let value: i32 = a
            .iter()
            .map(|i| (b.iter().filter(|j| **j == *i).count() as i32) * i)
            .sum();
        Ok(value as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
