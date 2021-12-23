use std::{fs::File, io::BufRead, io::BufReader};

type BitVec = Vec<bool>;

pub fn solve_part_1(input: &Vec<BitVec>) -> u32 {
    let bit_count = input[0].len();

    // Find most common bit for each position
    let gamma_rate_bits = (0..bit_count)
        .map(|i| input.iter().filter(|x| x[i] == true).count() > (input.len() / 2))
        .collect::<BitVec>();
    let gamma_rate = to_u32(&gamma_rate_bits);
    //println!("Gamma Rate Bits: {:?}", gamma_rate_bits);
    println!("Gamma Rate: {:?}", gamma_rate);

    // Negate to find least common bit for each position
    let epsilon_rate_bits = negate(&gamma_rate_bits);
    let epsilon_rate = to_u32(&epsilon_rate_bits);
    //println!("Epsilon Rate Bits: {:?}", epsilon_rate_bits);
    println!("Epsilon Rate: {:?}", epsilon_rate);

    return gamma_rate as u32 * epsilon_rate as u32;
}

pub fn solve_part_2(input: &Vec<BitVec>) -> usize {
    return 0;
}

fn to_u32(bits: &BitVec) -> u32 {
    return bits.iter().rev().enumerate().fold(0 as u32, |acc, (i, x)| {
        acc + (*x as u32) * u32::pow(2, i as u32)
    });
}

fn negate(bits: &BitVec) -> BitVec {
    return bits.iter().map(|x| !x).collect();
}

pub fn parse_input() -> Vec<BitVec> {
    let file = File::open("src/year_2021/day_03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut bits_collection = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let bits = line.chars().map(|c| c == '1').collect();
        bits_collection.push(bits);
    }
    return bits_collection;
}
