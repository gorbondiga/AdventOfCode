use std::fs;
use std::collections::HashMap;

// Union-Find (Disjoint Set Union) data structure
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return true;
        }
        
        // Union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }
    
    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }
}

#[derive(Clone, Debug)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl Position {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|coord| coord.parse().unwrap());

        Position {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }

    fn distance(&self, other: &Position) -> f64 {
        let dx = (self.x as f64) - (other.x as f64);
        let dy = (self.y as f64) - (other.y as f64);
        let dz = (self.z as f64) - (other.z as f64);
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn _part1(input: &str, connect: usize) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    
    // Parse positions
    let positions: Vec<Position> = input.lines().map(Position::from).collect();
    let n = positions.len();
    
    // Create all edges with distances
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();
    
    for i in 0..n {
        for j in (i+1)..n {
            let distance = positions[i].distance(&positions[j]);
            edges.push((distance, i, j));
        }
    }
    
    // Sort edges by distance (smallest first)
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("Sorted edges by distance: {:?}", edges);

    // Initialize Union-Find
    let mut uf = UnionFind::new(n);
    
    let mut connections_made = 0;
    
    for (_distance, i, j) in edges {
        if uf.union(i, j) {
            connections_made += 1;
            if connections_made == connect {
                break;
            }
        }
    }
    
    // Get all component sizes
    let mut sizes = uf.get_component_sizes();

    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending
    
    println!("\nCircuit sizes: {:?}", sizes);
    println!("Number of circuits: {}", sizes.len());
    
    // Multiply the three largest
    if sizes.len() >= 3 {
        let result = sizes[0] * sizes[1] * sizes[2];
        println!("\nThree largest circuits: {} × {} × {} = {}", 
                 sizes[0], sizes[1], sizes[2], result);
    }
}

fn part2(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    
    // Parse positions
    let positions: Vec<Position> = input.lines().map(Position::from).collect();
    let n = positions.len();
    
    // Create all edges with distances
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();
    
    for i in 0..n {
        for j in (i+1)..n {
            let distance = positions[i].distance(&positions[j]);
            edges.push((distance, i, j));
        }
    }
    
    // Sort edges by distance (smallest first)
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
    // Initialize Union-Find
    let mut uf = UnionFind::new(n);
    
    let mut last_connection = (0, 0); // Track the last connection made
    
    // Connect until all nodes are in ONE circuit
    for (_distance, i, j) in edges {
        if uf.union(i, j) {
            last_connection = (i, j);
            
            // Check if all nodes are now in one component
            let num_circuits = uf.get_component_sizes().len();
            // println!("Connected {} <-> {} (distance: {:.2}), circuits remaining: {}", 
            //          i, j, distance, num_circuits);
            
            // Stop when we have only 1 circuit (all nodes connected)
            if num_circuits == 1 {
                println!("\n✓ All {} nodes are now connected!", n);
                break;
            }
        }
    }
    
    let (box_i, box_j) = last_connection;
    let x1 = positions[box_i].x;
    let x2 = positions[box_j].x;
    let result = x1 * x2;
    
    println!("\nLast connection: Box {} at ({},{},{}) <-> Box {} at ({},{},{})",
             box_i, positions[box_i].x, positions[box_i].y, positions[box_i].z,
             box_j, positions[box_j].x, positions[box_j].y, positions[box_j].z);
    println!("X coordinates: {} × {} = {}", x1, x2, result);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    // _part1(_input_file, 1000);
    part2(_input_file);
}