use std::fs;

fn apply_operation(numbers: &[i64], op: &str) -> i64 {
    match op {
        "+" => numbers.iter().sum(),
        "*" => numbers.iter().product(),
        "-" => numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc - x),
        "/" => numbers.iter().skip(1).fold(numbers[0], |acc, &x| acc / x),
        _ => 0,
    }
}

fn _part1(input_file: &str) {
    let contents = fs::read_to_string(input_file)
                    .expect("Failed to read input file");
    let lines: Vec<&str> = contents.lines().collect();
    let operations = lines[lines.len() -1].split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
    let num_cols = lines[0].split_whitespace().count();
    let all_numbers: Vec<i64> = contents
        .split_whitespace()  // Splits by any whitespace (space, tab, newline)
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut l = 0;
    let mut result: i64 = 0;
    loop {
        let aux: Vec<i64> = all_numbers
                                .chunks(num_cols)
                                .map(|chunk| chunk[l])
                                .collect::<Vec<i64>>();
        let op = &operations[l];
        // println!("Column {}: {:?} with operation {}", l, aux, op);
        // println!("Result of operation: {}", apply_operation(&aux, op));
        result += apply_operation(&aux, op);
        l += 1;
        if l >= num_cols {
            break;
        }
    }
    println!("Final results for part 1: {}", result);
}

fn part2(input_file: &str) {
    let contents = fs::read_to_string(input_file)
                    .expect("Failed to read input file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut numbers: Vec<_> = lines[..lines.len() - 1].iter().map(|line| line.chars()).collect();
    let mut operators = lines.last().unwrap().chars();

    let mut result: u64 = 0;
    let mut curr: u64 = 0;
    let mut curr_op: char = ' ';
    while let Some(operator) = operators.next() {
        match operator {
                '+' => {
                    result += curr;
                    curr = 0;
                    curr_op = operator;
                }

                '*' => {
                    result += curr;
                    curr = 1;
                    curr_op = operator;
                }

                ' ' => {}

                _x => panic!("Unknown operator {}", _x),
        }

        let operand = numbers
                .iter_mut()
                .filter_map(|number| number.next())
                .filter_map(|x| x.to_digit(10))
                .map(|d| d as u64)
                .reduce(|acc, x| acc * 10 + x);

        match (curr_op, operand) {
                ('+', Some(operand)) => curr += operand,
                ('*', Some(operand)) => curr *= operand,
                _ => {}
        }
    }
    result += curr;

    println!("Final results for part 2: {}", result);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
    part2(_input_file);
}