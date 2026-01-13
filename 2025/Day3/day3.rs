use std::fs;

fn _joltage(numbers: &Vec<i64>) -> i64 {
    // Find largest number with the min index number 
    // which is not at the last position
    // second largest number starting from the index of the max 
    let len = numbers.len();
    let result: &mut Vec<i64> = &mut vec![];

    let mut current_index = 0;
    let mut k = 12;
    while k > 0 && current_index < len {
        let max = numbers[current_index..len-k+1].iter().max().unwrap();
        // Find the first (minimum) index of that max value
        let max_index = numbers[current_index..len-k+1]
            .iter()
            .position(|&x| x == *max)
            .unwrap();
        result.push(*max);
        // Convert to absolute index in the original array
        current_index = current_index + max_index + 1;
        k -= 1;
    }
    // Convert array [4, 2, 7] to number 427
    let combined = result.iter()
        .fold(0, |acc, &digit| acc * 10 + digit);
    return combined;
}

fn _part1(input: &str) {
    let contents = fs::read_to_string(input)
                    .expect("Failed to read input file");
    
    let digit_arrays: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let n = digit_arrays.len();
    let mut result: i64 = 0;
    for i in 0..n {
        let m = digit_arrays[i].len();
        let max_val = *digit_arrays[i][0..m-1].iter().max().unwrap();
        let max_idx = digit_arrays[i][0..m-1]
                        .iter()
                        .position(|&x| x == max_val)
                        .unwrap();
        let array_second_max = digit_arrays[i][max_idx+1..m].iter().max().unwrap();
        let combined_number = max_val * 10 + array_second_max;
        result += combined_number;
    }
    
    println!("Part1 result is {}", result);
}

fn part2(input: &str) {
    let contents = fs::read_to_string(input)
                    .expect("Failed to read input file");
    
    let digit_arrays: Vec<Vec<i64>> = contents
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let mut result: i64 = 0;
    for d in &digit_arrays {
        let x = _joltage(d);
        result += x;
    }

    println!("Part2 result is {}", result);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
    part2(_input_file);
}