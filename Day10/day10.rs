use std::fs;
use std::collections::{VecDeque, HashSet, HashMap};
use minilp::{Problem, OptimizationDirection, ComparisonOp};

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

fn combine_buttons_sum(result: &[usize], buttons: &[Vec<usize>]) -> Option<Vec<usize>> {
    // DP: dp[state] = (min_cost, button_that_got_us_here, previous_state)
    let mut dp: HashMap<Vec<usize>, (usize, Option<(usize, Vec<usize>)>)> = HashMap::new();
    let start = vec![0; result.len()];
    dp.insert(start.clone(), (0, None));
    
    let mut queue = VecDeque::new();
    queue.push_back(start);
    
    while let Some(current) = queue.pop_front() {
        let current_cost = dp[&current].0;
        
        // Limit depth
        if current_cost >= 15 {
            continue;
        }
        
        // Try each button
        for (i, button) in buttons.iter().enumerate() {
            let next: Vec<usize> = current.iter()
                .zip(button.iter())
                .map(|(a, b)| a + b)
                .collect();
            
            // Skip if overshoots
            if next.iter().zip(result.iter()).any(|(a, b)| a > b) {
                continue;
            }
            
            let new_cost = current_cost + 1;
            
            // Update if we found a better path or new state
            if !dp.contains_key(&next) || dp[&next].0 > new_cost {
                dp.insert(next.clone(), (new_cost, Some((i, current.clone()))));
                queue.push_back(next);
            }
        }
    }
    
    // Reconstruct path
    if !dp.contains_key(result) {
        return None;
    }
    
    let mut path = Vec::new();
    let mut current = result.to_vec();
    
    while let Some((button_idx, prev_state)) = &dp[&current].1 {
        path.push(*button_idx);
        current = prev_state.clone();
    }
    
    path.reverse();
    Some(path)
}

// Find coefficients: c0*button0 + c1*button1 + ... = result
fn combine_with_coefficients(result: &[usize], 
                            buttons: &[Vec<usize>]) 
                            -> Option<Vec<usize>> {
    let n = buttons.len();
    let max_coef = 100; // Max coefficient for each button
    
    // Convert to signed integers for calculation
    let target: Vec<usize> = result.to_vec();
    
    // Try all combinations of coefficients (brute force for small n)
    fn search(
        buttons: &[Vec<usize>],
        target: &[usize],
        coefficients: &mut Vec<usize>,
        index: usize,
        max_coef: usize,
    ) -> bool {
        if index == buttons.len() {
            // Check if this combination works
            let mut sum = vec![0usize; target.len()];
            for (i, coef) in coefficients.iter().enumerate() {
                for (j, &val) in buttons[i].iter().enumerate() {
                    sum[j] += coef * val;
                }
            }
            return sum == target;
        }
        
        // Try different coefficients for this button
        for coef in 0..=max_coef {
            coefficients.push(coef);
            if search(buttons, target, coefficients, index + 1, max_coef) {
                return true;
            }
            coefficients.pop();
        }
        false
    }
    
    let mut coefficients = Vec::new();
    if search(buttons, &target, &mut coefficients, 0, max_coef) {
        Some(coefficients)
    } else {
        None
    }
}

// Integer Linear Programming solver using minilp
fn solve_ilp(result: &[usize], buttons: &[Vec<usize>]) -> Option<Vec<usize>> {
    let num_buttons = buttons.len();
    let num_positions = result.len();
    
    // Create problem: minimize sum of all button presses
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    
    // Create variables for each button (how many times to press)
    let vars: Vec<_> = (0..num_buttons)
        .map(|_| problem.add_var(1.0, (0.0, f64::INFINITY)))
        .collect();
    
    // Add constraints: for each position, sum(button[i][pos] * x[i]) == result[pos]
    for pos in 0..num_positions {
        let coefficients: Vec<(_, f64)> = vars.iter()
            .enumerate()
            .map(|(btn_idx, &var)| (var, buttons[btn_idx][pos] as f64))
            .collect();
        
        problem.add_constraint(
            &coefficients,
            ComparisonOp::Eq,
            result[pos] as f64
        );
    }
    
    // Solve
    let solution = problem.solve().ok()?;
    
    // Extract integer coefficients
    let coefficients: Vec<usize> = vars.iter()
        .map(|&v| solution[v].round() as usize)
        .collect();
    
    // Verify solution is valid
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
                .map(|x| x.len())
                .sum::<usize>());
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
    part2(_input_file);
}