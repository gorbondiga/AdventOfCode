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

    fn find_paths_from_start_dfs(&self, start: &str) -> HashSet<Vec<String>> {
        let mut all_paths = HashSet::new();
        let mut stack = Vec::new();
        
        // Track (path, visited_set) to avoid O(n) contains checks
        let mut initial_visited = HashSet::new();
        initial_visited.insert(start.to_string());
        stack.push((vec![start.to_string()], initial_visited));
        
        while let Some((path, visited)) = stack.pop() {
            let current = path.last().unwrap();
            
            if current == "out" {
                let has_dac = path.iter().any(|n| n == "dac");
                let has_fft = path.iter().any(|n| n == "fft");
                
                if has_dac && has_fft {
                    all_paths.insert(path.clone());
                }
                continue;
            }
            
            if let Some(connections) = self.get_connections(current) {
                for conn in connections {
                    // O(1) cycle detection with HashSet
                    if !visited.contains(conn) {
                        let mut new_path = path.clone();
                        new_path.push(conn.clone());
                        
                        let mut new_visited = visited.clone();
                        new_visited.insert(conn.clone());
                        
                        stack.push((new_path, new_visited));
                    }
                }
            }
        }
        all_paths
    }

    fn count_paths_dfs(&self, start: &str) -> usize {
        // Count paths through dac first, then fft
        let count1 = self.count_segment_paths(start, "dac") 
            * self.count_segment_paths("dac", "fft")
            * self.count_segment_paths("fft", "out");
        
        // Count paths through fft first, then dac
        let count2 = self.count_segment_paths(start, "fft")
            * self.count_segment_paths("fft", "dac")
            * self.count_segment_paths("dac", "out");
        
        count1 + count2
    }

    fn count_segment_paths(&self, start: &str, end: &str) -> usize {
        let mut memo: HashMap<String, usize> = HashMap::new();
        self.count_paths_memo(start, end, &mut HashSet::new(), &mut memo)
    }

    fn count_paths_memo(&self, current: &str, target: &str, visited: &mut HashSet<String>, memo: &mut HashMap<String, usize>) -> usize {
        if current == target {
            return 1;
        }

        // Check memo (only valid if this node isn't in current path)
        if !visited.contains(current) {
            if let Some(&cached) = memo.get(current) {
                return cached;
            }
        }

        if visited.contains(current) {
            return 0;
        }

        visited.insert(current.to_string());
        
        let mut total = 0;
        if let Some(connections) = self.get_connections(current) {
            for conn in connections {
                total += self.count_paths_memo(conn, target, visited, memo);
            }
        }

        visited.remove(current);
        
        // Only memoize if we can reach this state again without visiting current
        if total > 0 {
            memo.insert(current.to_string(), total);
        }
        
        total
    }

    fn dfs_count(&self, current: &str, visited: &mut HashSet<String>, has_dac: bool, has_fft: bool, count: &mut usize) {
        if current == "out" {
            if has_dac && has_fft {
                *count += 1;
            }
            return;
        }

        visited.insert(current.to_string());

        let new_has_dac = has_dac || current == "dac";
        let new_has_fft = has_fft || current == "fft";

        if let Some(connections) = self.get_connections(current) {
            for conn in connections {
                if !visited.contains(conn) {
                    self.dfs_count(conn, visited, new_has_dac, new_has_fft, count);
                }
            }
        }

        visited.remove(current);
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

    let count = graph.count_paths_dfs("svr");

    println!("Result for part2: {}", count);
}

fn main() {
    let _input_file = "input.txt";
    // let _input_file = "input_test.txt";
    // let _input_file = "input_test_2.txt";
    // _part1(_input_file);
    part2(_input_file);
}