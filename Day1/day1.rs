use std::fs;

fn convert_string(value: &str) -> i32 {
    let direction = &value[0..1];
    let number: i32 = value[1..].parse().expect("Failed to parse number");
    
    if direction == "R" {
        number
    } else {
        -number
    }
}

fn main() {
    let input_file = "input.txt";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let mut part_two: i32 = 0;
    let mut pos: i32 = 50;
    let mut _prev: i32;
    
    for line in contents.lines() {
        _prev = pos;
        let dir: i32 = if line.chars().next().unwrap() == 'R' { 1 } else { -1 };
        let rot_value = convert_string(line.trim());

        let sum: i32 = pos + rot_value;
        
        // Add 1 if 100 or more is crossed
        part_two += sum.abs() / 100;

        // Handle left direction special cases
        if dir < 0 {
            let sum_remainder: i32 = sum % 100; // error because the sum is 300 and the remainder 0
            let aux: i32 = pos.div_euclid(100) + (sum_remainder).div_euclid(100);
            pos = sum_remainder.rem_euclid(100);
            if _prev != 0 {
                part_two += aux.abs();
            }
            if pos == 0 {
                part_two += 1;
            }
        } else {
            pos = sum.abs() % 100;
        }
    }
    println!("======== Final result ========");
    println!("Part two: {}", part_two);
}