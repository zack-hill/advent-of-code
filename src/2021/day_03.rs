use crate::solver::AoCSolver;
use std::{fs::File, io::BufRead, io::BufReader};

type BitVec = Vec<bool>;

pub struct Solver {
    input: Vec<BitVec>,
}

impl Solver {
    pub fn create() -> Self {
        Solver {
            input: parse_input(),
        }
    }
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let bit_count = self.input[0].len();

        // Find most common bit for each position
        let gamma_rate_bits = (0..bit_count)
            .map(|i| get_most_common_bit(&self.input, i))
            .collect::<BitVec>();
        let gamma_rate = to_u32(&gamma_rate_bits);
        //println!("Gamma Rate Bits: {:?}", gamma_rate_bits);
        // println!("Gamma Rate: {:?}", gamma_rate);

        // Negate to find least common bit for each position
        let epsilon_rate_bits = negate(&gamma_rate_bits);
        let epsilon_rate = to_u32(&epsilon_rate_bits);
        //println!("Epsilon Rate Bits: {:?}", epsilon_rate_bits);
        // println!("Epsilon Rate: {:?}", epsilon_rate);

        return (gamma_rate as u32 * epsilon_rate as u32).to_string();
    }

    fn solve_part_2(&self) -> String {
        let bit_count = self.input[0].len();

        let mut working_set = self.input.clone();
        // println!("Working Set: {:?}", working_set);
        for position in 0..bit_count {
            // println!("Position: {}", position);
            let most_common_bit = get_most_common_bit(&working_set, position);
            // println!("Most Common Bit: {}", most_common_bit);
            working_set.drain_filter(|x| x[position] != most_common_bit);
            if working_set.len() == 1 {
                break;
            }
            // println!("New Working Set: {:?}", working_set);
        }
        let oxygen_rating_bits = working_set.get(0).unwrap();
        let oxygen_rating = to_u32(oxygen_rating_bits);
        // println!("Oxygen Rating Bits: {:?}", oxygen_rating_bits);
        // println!("Oxygen Rating: {:?}", oxygen_rating);

        let mut working_set = self.input.clone();
        // println!("Working Set: {:?}", working_set);
        for position in 0..bit_count {
            // println!("Position: {}", position);
            let most_common_bit = get_least_common_bit(&working_set, position);
            // println!("Least Common Bit: {}", most_common_bit);

            working_set.drain_filter(|x| x[position] != most_common_bit);
            if working_set.len() == 1 {
                break;
            }
            // println!("New Working Set: {:?}", working_set);
        }
        let c02_rating_bits = working_set.get(0).unwrap();
        let c02_rating = to_u32(c02_rating_bits);
        // println!("C02 Rating Bits: {:?}", c02_rating_bits);
        // println!("C02 Rating: {:?}", c02_rating);

        return (oxygen_rating * c02_rating).to_string();
    }
}

fn get_most_common_bit(input: &Vec<BitVec>, position: usize) -> bool {
    (input.iter().filter(|x| x[position] == true).count() as f32) >= (input.len() as f32 / 2f32)
}

fn get_least_common_bit(input: &Vec<BitVec>, position: usize) -> bool {
    (input.iter().filter(|x| x[position] == true).count() as f32) < input.len() as f32 / 2f32
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
    let file = File::open("src/2021/day_03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut bits_collection = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let bits = line.chars().map(|c| c == '1').collect();
        bits_collection.push(bits);
    }
    return bits_collection;
}
