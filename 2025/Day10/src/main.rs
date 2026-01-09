use std::fs;
use std::collections::{VecDeque, HashSet};
use good_lp::*;

#[derive(Debug)]
struct Factory {
    result: Vec<usize>,
    buttons: Vec<Vec<usize>>,
}

fn combine_buttons(result: &[usize], 
                    buttons: &[Vec<usize>]) 
    -> Option<Vec<usize>> {
    let n = buttons.len();
    
    // BFS: (current_state, used_buttons_mask)
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    // Start with empty state (all zeros)
    queue.push_back((vec![0; result.len()], 0u64));
    visited.insert(0u64);
    
    while let Some((current, mask)) = queue.pop_front() {
        // Check if we reached the target
        if current == result {
            // Extract which buttons were used from the mask
            let indices: Vec<usize> = (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .collect();
            return Some(indices);
        }
        
        // Try pressing each unused button
        for i in 0..n {
            if mask & (1 << i) != 0 {
                continue;
            }
            
            // XOR current state with button[i]
            let next_state: Vec<usize> = current.iter()
                .zip(buttons[i].iter())
                .map(|(a, b)| a ^ b)
                .collect();
            
            let next_mask = mask | (1 << i);
            
            if visited.insert(next_mask) {
                queue.push_back((next_state, next_mask));
            }
        }
    }
    
    None
}

// Integer Linear Programming solver using minilp
fn solve_ilp(result: &[usize], 
            buttons: &[Vec<usize>]) -> Option<Vec<usize>> {
    let num_buttons = buttons.len();
    let num_positions = result.len();
    
    // Create variables
    let mut vars = ProblemVariables::new();
    let button_vars: Vec<Variable> = (0..num_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();
    
    // Objective: minimize total button presses
    let objective: Expression = button_vars.iter().copied().sum();
    
    // Build problem
    let mut problem = vars.minimise(objective).using(coin_cbc);
    
    // Add constraints: for each position
    for pos in 0..num_positions {
        let constraint_expr: Expression = button_vars.iter()
            .enumerate()
            .map(|(btn_idx, &var)| (buttons[btn_idx][pos] as i32) * var)
            .sum();

        problem = problem.with(constraint_expr.eq(result[pos] as i32));

    }
    
    // Solve
    let solution = problem.solve().ok()?;
    
    // Extract integer coefficients
    let coefficients: Vec<usize> = button_vars.iter()
        .map(|&v| solution.value(v) as usize)
        .collect();

    println!("Coefficients: {:?}", coefficients);
    
    // Verify solution
    for pos in 0..num_positions {
        let sum: usize = coefficients.iter()
            .enumerate()
            .map(|(i, &c)| c * buttons[i][pos])
            .sum();
        if sum != result[pos] {
            return None;
        }
    }
    
    Some(coefficients)
}

fn _part1(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    
    let list: Vec<Factory> = input.lines()
        .filter_map(|line| {
            // Find the pattern between []
            let start = line.find('[')?;
            let end = line.find(']')?;
            let pattern = &line[start + 1..end];
            
            let result: Vec<usize> = pattern.chars()
                .map(|ch| if ch == '#' { 1 } else { 0 })
                .collect();

            // Parse buttons - everything between ] and {
            let buttons_part = if let Some(brace_pos) = line.find('{') {
                &line[end + 1..brace_pos]
            } else {
                &line[end + 1..]
            };

            let mut buttons: Vec<Vec<usize>> = Vec::new();
            let mut in_parens = false;
            let mut current_group = String::new();
            
            for ch in buttons_part.chars() {
                match ch {
                    '(' => {
                        in_parens = true;
                        current_group.clear();
                    },
                    ')' => {
                        in_parens = false;
                        
                        let indices: Vec<usize> = current_group
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();
                        
                        if !indices.is_empty() {
                            // Create vec and mark indices as 1
                            let binary = (0..result.len())
                                .map(|i| if indices.contains(&i) { 1 } else { 0 })
                                .collect();
                            
                            buttons.push(binary);
                        }
                    },
                    _ if in_parens => {
                        current_group.push(ch);
                    },
                    _ => {}
                }
            }

            Some(Factory {
                result,
                buttons,
            })
        })
        .collect();
    
    let result: Vec<Option<Vec<usize>>> = list.iter()
                            .map(|factory| combine_buttons(&factory.result, 
                                                            &factory.buttons))
                            .collect();

    println!("Part 1 Results: {}", 
            result.iter()
            .filter_map(|x| x.as_ref())
            .map(|x| x.len()).sum::<usize>());
}


fn part2(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    
    let list: Vec<Factory> = input.lines()
        .filter_map(|line| {
            // Find the pattern between {}
            let start = line.find('{')?;
            let end = line.find('}')?;
            let pattern = &line[start + 1..end];
            
            let result: Vec<usize> = pattern.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            
            // Parse buttons - everything with ()
            let mut buttons: Vec<Vec<usize>> = Vec::new();
            let mut in_parens = false;
            let mut current_group = String::new();
            
            for ch in line.chars() {
                match ch {
                    '(' => {
                        in_parens = true;
                        current_group.clear();
                    },
                    ')' => {
                        in_parens = false;
                        
                        let indices: Vec<usize> = current_group
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();
                        
                        if !indices.is_empty() {
                            // Create vec and mark indices as 1
                            let binary = (0..result.len())
                                .map(|i| if indices.contains(&i) { 1 } else { 0 })
                                .collect();
                            
                            buttons.push(binary);
                        }
                    },
                    _ if in_parens => {
                        current_group.push(ch);
                    },
                    _ => {}
                }
            }

            Some(Factory {
                result,
                buttons,
            })
        })
        .collect();

    let result: Vec<Option<Vec<usize>>> = list.iter()
                            .map(|factory| solve_ilp(&factory.result, 
                                                            &factory.buttons))
                            .collect();

    println!("Part 2 Results: {}", 
            result.iter()
                .filter_map(|x| x.as_ref())
                .map(|coefficients| coefficients.iter().sum::<usize>())
                .sum::<usize>());
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
    part2(_input_file);
}