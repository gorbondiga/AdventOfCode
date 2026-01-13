use std::fs;

fn _adjacent(r: i32, c: i32, 
            matrix: &Vec<Vec<i32>>) 
            -> i32 {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let neighbors = vec![
        (c - 1, r - 1),
        (c - 1, r),
        (c - 1, r + 1),
        (c, r - 1),
        (c, r + 1),
        (c + 1, r - 1),
        (c + 1, r),
        (c + 1, r + 1),
    ];
    let mut sum = 0;
    for (nc, nr) in neighbors {
        if nc >= 0 && nr < rows && nr >= 0 && nc < cols {
            sum += matrix[nr as usize][nc as usize];
        }
    }
    return sum;
}

fn part_2(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let nr = matrix.len();
    let nc = matrix[0].len();
    let mut result = 0;
    for row in 0..nr {
        for col in 0..nc {
            let aux = _adjacent(row as i32, 
                                col as i32, &matrix);
            if aux < 4 && matrix[row][col] == 1 {
                matrix[row][col] = 0;
                result += 1;
            }
        }
    }
    result
}

fn _part1(input: &str) {
    let contents = fs::read_to_string(input)
                    .expect("Failed to read input file");
    let matrix: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();
    let w = matrix[0].len();
    let h = matrix.len();
    let mut result = 0;
    for c in 0..w {
        for r in 0..h {
            let _adj = _adjacent(r as i32, c as i32, &matrix);
            if _adj < 4 && matrix[r][c] == 1 {
                result += 1;
            }
        }
    }  
    println!("Part1 result is {}", result);
}

fn part2(input: &str) {
    let contents = fs::read_to_string(input)
                    .expect("Failed to read input file");
    let mut matrix: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();
    let mut num_changes = 0;
    let result = loop {  
        let changes = part_2(&mut matrix);
        num_changes += changes;
        if  changes == 0 {
            break num_changes;
        }
    };
    println!("Part2 result is {}", result);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
    part2(_input_file);
}