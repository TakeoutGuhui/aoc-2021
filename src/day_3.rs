use crate::utils::read_lines;

pub fn part_1() {
    let lines = read_lines("input/day_3.txt").unwrap(); 
    let numbers: Vec<u32> = lines.iter().map(|line| u32::from_str_radix(line, 2).unwrap()).collect();
    let mut counts: [u32; 12] = [0; 12];

    for number in &numbers {
        for i in 0..12 {
            counts[i] += (number >> i) & 1;  
        } 
    }
    let mut number: u32 = 0;
    let half_size = numbers.len() / 2;
    for i in 0..12 {
        let bit = if counts[i] > half_size as u32 { 1 } else { 0 }; 
        number |= bit << i; 
    }
    let flipped_number = !number & 0b1111_1111_1111; 
    let result = number as u32 * flipped_number as u32;
    println!("   Part 1: {}", result);
}
