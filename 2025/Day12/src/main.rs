use std::fs;
use std::collections::HashSet;
use ndarray::{Array2, s};

#[derive(Debug, Clone)]
struct Shape {
    grid: Array2<u8>,
}

impl Shape {
    fn grid_to_bitmask(grid: &Array2<u8>) -> Vec<u64> {
        grid.rows()
            .into_iter()
            .map(|row| {
                let mut mask = 0u64;
                for (i, &val) in row.iter().enumerate() {
                    if val == 1 {
                        mask |= 1 << i;
                    }
                }
                mask
            })
            .collect()
    }

    fn rotate_90(grid: &Array2<u8>) -> Array2<u8> {
        grid.t().slice(s![.., ..;-1]).to_owned()
    }

    fn get_unique_bitmasks(&self) -> Vec<Vec<u64>> {
        let mut unique_set = HashSet::new();
        let mut current = self.grid.clone();
        
        for _ in 0..4 {
            let mask = Shape::grid_to_bitmask(&current);
            unique_set.insert(mask);
            current = Shape::rotate_90(&current);
        }
        
        unique_set.into_iter().collect()
    }
}

#[derive(Debug)]
struct BitmaskPacker {
    width: usize,
    length: usize,
    shape_ids: Vec<usize>,
}

impl BitmaskPacker {
    fn can_fit_volume(&self, 
            base_shapes: &[Shape]) -> bool {
        let mut total_ones: usize = 0;

        for (id, &count) in self.shape_ids.iter().enumerate() {
            if count > 0 {
                let ones = base_shapes[id].grid.iter()
                            .filter(|&&x| x == 1).count(); 
                
                total_ones += ones * count;
            }
        }

        let grid_capacity = self.width * self.length;
        
        total_ones <= grid_capacity
    }

    fn solve_recursive(&self, grid: &mut Vec<u64>, 
                            inventory: &mut Vec<Vec<Vec<u64>>>) -> bool {
        if inventory.is_empty() {
            return true;
        }

        let current_shape_rotations = inventory.pop().unwrap();

        for rotation in &current_shape_rotations {
            let h = rotation.len();
            let w = rotation.iter()
                .map(|row| (64 - row.leading_zeros()) as usize)
                .max().unwrap_or(0);

            // Boundary Check
            if h > self.length || w > self.width { continue; }

            // Try every possible (x, y) coordinate
            for y in 0..=(self.length - h) {
                for x in 0..=(self.width - w) {
                    
                    // Check if it overlaps with what's already in the grid
                    let mut fits = true;
                    for (i, row_mask) in rotation.iter().enumerate() {
                        if (grid[y + i] & (row_mask << x)) != 0 {
                            fits = false;
                            break;
                        }
                    }

                    if fits {
                        // Place (STAMP)
                        for (i, row_mask) in rotation.iter().enumerate() {
                            grid[y + i] |= row_mask << x;
                        }

                        // Recurse to next shape
                        if self.solve_recursive(grid, inventory) {
                            return true;
                        }

                        // Backtrack (REMOVE)
                        for (i, row_mask) in rotation.iter().enumerate() {
                            grid[y + i] &= !(row_mask << x);
                        }
                    }
                }
            }
        }

        // If we tried all rotations and positions and nothing worked, 
        // put the shape back and return false.
        inventory.push(current_shape_rotations);
        false
    }

    pub fn can_physically_fit(&self, rotations_cache: &Vec<Vec<Vec<u64>>>) -> bool {
        let mut inventory = Vec::new();
        for (id, &count) in self.shape_ids.iter().enumerate() {
            for _ in 0..count {
                inventory.push(rotations_cache[id].clone());
            }
        }

        // Sort biggest shapes first (optimization)
        inventory.sort_by_key(|r| r[0].iter().map(|line| line.count_ones()).sum::<u32>());
        inventory.reverse();

        let mut grid = vec![0u64; self.length];

        self.solve_recursive(&mut grid, &mut inventory)
    }

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
            let mut grid_data = Vec::new();
            
            // Read the 3 rows of the shape
            while i < lines.len() && !lines[i].trim().is_empty() {
                let row: Vec<u8> = lines[i]
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect();
                grid_data.push(row);
                i += 1;
            }
            
            if !grid_data.is_empty() {
                let rows = grid_data.len();
                let cols = grid_data[0].len();
                let flat: Vec<u8> = grid_data.into_iter().flatten().collect();
                let grid = Array2::from_shape_vec((rows, cols), flat).unwrap();
                shapes.push(Shape { grid });
            }
        }
        i += 1;
    }
    
    shapes
}

fn read_rest_of_file(input: &str) -> Vec<BitmaskPacker> {
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
                        
                        regions.push(BitmaskPacker {
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
    let base_shapes = read_shapes(input, 6);
    
    let rotations_cache: Vec<Vec<Vec<u64>>> = base_shapes.iter()
        .map(|s| {
            s.get_unique_bitmasks() 
        }).collect();

    let mut regions = read_rest_of_file(input);

    regions.retain(|b| {
        b.can_fit_volume(&base_shapes)
    });

    let mut result = 0;
    for region in &regions {
        if region.can_physically_fit(&rotations_cache) {
            result += 1;
        }
    }

    println!("Part 1: {}", result);

}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
}