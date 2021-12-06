use crate::utils::read_lines;

fn calculate_depth_inc(data: &[u32], step: usize) -> u32 {
    let mut count = 0;

    for i in 0..data.len() - step {
        if data[i + step] > data[i] {
            count += 1;
        }
    }
    count
}

pub fn part_1_2() {
    let input = read_lines("input/day_1.txt").unwrap();
    let input: Vec<u32> = input.iter().map(|line| line.parse().unwrap()).collect();

    let result_1 = calculate_depth_inc(&input, 1);
    println!("Result 1: {}", result_1);
    let result_2 = calculate_depth_inc(&input, 3);
    println!("Result 2: {}", result_2);
}
