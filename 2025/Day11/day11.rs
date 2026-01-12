use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Node {
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

    fn dfs_memo(
        &self,
        u: String,
        target: &str,
        req1: &str,
        req2: &str,
        mut f1: bool,
        mut f2: bool,
        visited: &mut HashSet<String>,
        memo: &mut HashMap<(String, bool, bool), usize>
    ) -> usize {
        if u == req1 { f1 = true; }
        if u == req2 { f2 = true; }

        if u == target {
            return if f1 && f2 { 1 } else { 0 };
        }

        // Check cache
        let state = (u.clone(), f1, f2);
        if let Some(&count) = memo.get(&state) {
            return count;
        }

        visited.insert(u.clone());
        let mut total_paths = 0;

        if let Some(node_data) = self.nodes.get(&u) {
            for neighbor in &node_data.connections {
                if !visited.contains(neighbor) {
                    total_paths += self.dfs_memo(
                        neighbor.clone(),
                        target,
                        req1,
                        req2,
                        f1,
                        f2,
                        visited,
                        memo
                    );
                }
            }
        }

        visited.remove(&u);
        
        memo.insert(state, total_paths);
        total_paths
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

 fn part2(input: &str) {
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

    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    let mut visited_in_current_path = HashSet::new();

    let count_paths = graph.dfs_memo("svr".to_string(), "out", "fft", "dac",
                                        false, false, &mut visited_in_current_path,
                                        &mut memo);

    println!("Result for part2: {}", count_paths);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    // let _input_file = "input_test_2.txt";
    _part1(_input_file);
    part2(_input_file);
}