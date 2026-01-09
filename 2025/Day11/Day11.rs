use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    connections: Vec<String>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }
    
    fn add_node(&mut self, name: String, connections: Vec<String>) {
        self.nodes.insert(name.clone(), Node { 
            name, 
            connections,
        });
    }
    
    fn get_connections(&self, node: &str) -> Option<&Vec<String>> {
        self.nodes.get(node).map(|n| &n.connections)
    }

    fn find_paths_from_start_bfs(&self, start: &str) -> HashSet<Vec<String>> {
        let mut all_paths = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with the initial node
        queue.push_back(vec![start.to_string()]);
        
        while let Some(path) = queue.pop_front() {
            let current = path.last().unwrap();
            
            // Check if we've reached "out"
            if current == "out" {
                all_paths.insert(path.clone());
                continue;
            }
            
            // Explore all connections
            if let Some(connections) = self.get_connections(current) {
                for conn in connections {
                    // Skip if already in path (avoid cycles)
                    if !path.contains(conn) {
                        let mut new_path = path.clone();
                        new_path.push(conn.clone());
                        queue.push_back(new_path);
                    }
                }
            }
        }
        
        all_paths
    }
    
}

fn _part1(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    let mut graph = Graph::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let node_name = parts[0].trim().to_string();
        let connections: Vec<String> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        graph.add_node(node_name, connections);
    }

    let mut all_paths = HashSet::new();
    let paths = graph.find_paths_from_start_bfs("you");
    all_paths.extend(paths);

    println!("Result for part1: {}", all_paths.len());
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    _part1(_input_file);
}