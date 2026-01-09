use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FreshInfo {
    start: i64,
    end: i64,
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    let contents = fs::read_to_string(_input_file)
                    .expect("Failed to read input file");
        // Split by empty line
    let file: Vec<&str> = contents.split("\n\n").collect();
    
    // First list (before empty line)
    let fresh: Vec<&str> = file[0].lines().collect();

    // Second list (after empty line)
    let ingredients: Vec<&str> = file[1].lines().collect();

    let mut result: HashSet<i64> = HashSet::new();
    for i in ingredients.iter() {
        let id: i64 = i.parse().expect("Invalid number");
        
        for f in fresh.iter() {
            let f_range: Vec<&str> = f.split('-').collect();
            if id >= f_range[0].parse().unwrap() &&
               id <= f_range[1].parse().unwrap() {
                result.insert(id);
            }
        }   
    }
    println!("Result for part 1: {:?}", result.len());

    let mut fresh_infos: Vec<FreshInfo> = Vec::new();
    for f in fresh.iter() {
        // Split and destructure in one line
        let f_range_2: Vec<&str> = f.split('-').collect();
        let start: i64 = f_range_2[0].parse().unwrap();
        let end: i64 = f_range_2[1].parse().unwrap();
        
        let fresh_info = FreshInfo { start, end };
        fresh_infos.push(fresh_info);
    }
    fresh_infos.sort();

    let mut stack: Vec<FreshInfo> = Vec::new();
    stack.push(fresh_infos[0].clone());
    for fi in fresh_infos.iter_mut() {
        // Check all overlap cases
        if fi.start >= stack.last().unwrap().start && 
            fi.start <= stack.last().unwrap().end {
            stack.last_mut().unwrap().end = stack.last().unwrap().end.max(fi.end);
        } else {
            stack.push(fi.clone());
        }
    }
    let mut result_2 = 0;
    for s in stack.iter() {
        result_2 += s.end - s.start + 1;
    }
    println!("Result for part 2: {:?}", result_2);
}
