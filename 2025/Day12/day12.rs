use std::fs;

#[derive(Debug)]
struct Shape {
    grid: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    length: usize,
    shape_ids: Vec<usize>,
}

fn read_shapes(input: &str, num_shapes: usize) -> Vec<Shape> {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    
    let mut shapes = Vec::new();
    let mut i = 0;
    
    while shapes.len() < num_shapes && i < lines.len() {
        // Skip until we find a shape number line (e.g., "0:")
        if lines[i].trim().ends_with(':') {
            i += 1; // Move to the first row of the shape
            let mut grid = Vec::new();
            
            // Read the 3 rows of the shape
            while i < lines.len() && !lines[i].trim().is_empty() {
                let row: Vec<u8> = lines[i]
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect();
                grid.push(row);
                i += 1;
            }
            
            if !grid.is_empty() {
                shapes.push(Shape { grid });
            }
        }
        i += 1;
    }
    
    shapes
}

fn read_rest_of_file(input: &str) -> Vec<Region> {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
    
    let mut i = 0;
    
    // Skip until we find a line with 'x' (marks the end of shapes section)
    while i < lines.len() && !lines[i].contains('x') {
        i += 1;
    }
    
    let mut regions = Vec::new();
    while i < lines.len() {
        let line = lines[i].trim();
        if !line.is_empty() && line.contains('x') {
            // Parse line like "37x43: 28 31 34 28 24 23"
            if let Some((dimensions, ids_str)) = line.split_once(':') {
                if let Some((w_str, l_str)) = dimensions.split_once('x') {
                    if let (Ok(width), Ok(length)) = (w_str.trim().parse(), l_str.trim().parse()) {
                        let shape_ids: Vec<usize> = ids_str
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        
                        regions.push(Region {
                            width,
                            length,
                            shape_ids,
                        });
                    }
                }
            }
        }
        i += 1;
    }
    
    regions
}

fn _part1(input: &str) {
    let shapes = read_shapes(input, 6);
    println!("Shapes: {:?}\n", shapes);
    
    let regions = read_rest_of_file(input);
    println!("Regions: {:?}\n", regions);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
}