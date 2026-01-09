use std::fs;
use std::collections::{HashSet, HashMap};
use std::str::Lines;



fn _part1(input: &str) {

    let input = fs::read_to_string(input).expect("Failed to read file!");
    let mut tachyon_beams: HashSet<usize> = HashSet::new();

    let char_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut split_count = 0;

    for line in char_lines {
        
        if line.contains(&'S') { // Read the first line
            tachyon_beams.insert(line.iter()
                            .position(|c| *c == 'S')
                            .unwrap());
            continue;
        }

        let mut changes: Vec<(usize, usize, usize)> = vec![];
        for beam in &tachyon_beams {
            if line[*beam] == '^' {
                split_count += 1;
                let left_point = beam - 1;
                let right_point = beam + 1;
                changes.push((left_point, right_point, *beam));
            }
        }

        for (left, right, to_remove) in changes {
            tachyon_beams.remove(&to_remove);
            tachyon_beams.insert(left);
            tachyon_beams.insert(right);

        }
    }
    println!("Final split count: {}", split_count);
}

fn part2(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");

    // Convert all lines to Vec<char> for indexing
    let lines: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    // Get the first line and find position of 'S'
    let mut beams = HashMap::new();
    if let Some(first_line) = lines.first() {
        if let Some(s_position) = first_line.iter().position(|&c| c == 'S') {
            beams.insert(s_position, 1);
        }
    }

    // Iterate over lines
    for line in lines.iter().skip(1) {
        let mut next_beams = HashMap::new();
        for (idx, count) in beams {
            if line[idx] == '^' {
                *next_beams.entry(idx - 1).or_insert(0) += count;
                *next_beams.entry(idx + 1).or_insert(0) += count;
            } else {
                *next_beams.entry(idx).or_insert(0) += count;
            }
        }
        beams = next_beams;
    }

    let result: usize = beams.values().sum();
    println!("Total paths: {}", result);

}

fn recursive_part2(lines_iter: &mut Lines, 
                  incoming_beams: HashMap<usize, usize>) 
                  -> usize {
    if let Some(line) = lines_iter.next() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut next_beams = HashMap::new();
        for (k, v) in incoming_beams {
            if chars[k] == '^' {
                *next_beams.entry(k - 1).or_insert(0) += v;
                *next_beams.entry(k + 1).or_insert(0) += v;
            } else {
                *next_beams.entry(k).or_insert(0) += v;
            }
        }
        recursive_part2(lines_iter, next_beams)
    } else {
        incoming_beams.values().sum::<usize>()
    }
}

fn _part2(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let result: usize = recursive_part2(
                            &mut lines,
                            HashMap::from([(first_line.find('S').unwrap(), 1)]),
                        );
    println!("Total paths: {}", result);
}


fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    // _part1(_input_file);
    part2(_input_file);
}