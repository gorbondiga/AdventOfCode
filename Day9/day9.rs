use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn area(&self, other: &Coord) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;
        dx * dy
    }
}

fn _part1(input: &str) {
    let input = fs::read_to_string(input).expect("Failed to read file!");
    
    // Parse positions
    let positions: Vec<Coord> = input.lines().map(|line| {
                let mut coords = line.split(',').map(|coord| coord.parse().unwrap());
                Coord {
                    x: coords.next().unwrap(),
                    y: coords.next().unwrap(),
                }
            }).collect();
    
    let n = positions.len();
    let mut area = 0;
    let mut result = (0, 0);

    for i in 0..n {
        for j in (i+1)..n {
            let aux = positions[i].area(&positions[j]);

            if aux > area && i != j {
                area = aux;
                result = (i, j);
            }
        }
    }

    println!("\nMax area is {} between point ({}, {}) and ({}, {})", 
             area, 
             positions[result.0].x, positions[result.0].y,
             positions[result.1].x, positions[result.1].y);
    
}

fn part2(input: &str) {
    let input = std::fs::read_to_string(input).expect("Failed to read file!");
    let corners: Vec<Coord> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split(',');
            let x = parts.next()?.trim().parse().ok()?;
            let y = parts.next()?.trim().parse().ok()?;
            Some(Coord { x, y })
        })
        .collect();
    let n = corners.len();

    let mut edges = Vec::new();
    let mut sizes = Vec::new();

    for i in 0..n {
        let mut edge = [corners[i], corners[(i + n - 1) % n]];
        edge.sort();
        edges.push(edge);
        for j in (i + 1)..n {
            let mut c1 = corners[i];
            let mut c2 = corners[j];
            if c2 < c1 {
                std::mem::swap(&mut c1, &mut c2);
            }
            let size = c1.area(&c2);
            sizes.push((size, c1, c2));
        }
    }

    edges.sort_by_key(|edge| (-edge[0].x, -edge[1].x));
    sizes.sort_by_key(|&(size, _, _)| -size);

    println!("Sizes: {:?}", sizes);
    
    for (size, c1, c2) in &sizes {
        let (x1, y1) = (c1.x, c1.y);
        let (x2, y2) = (c2.x, c2.y);
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        let mut found = false;
        for edge in &edges {
            let (x3, y3) = (edge[0].x, edge[0].y);
            let (x4, y4) = (edge[1].x, edge[1].y);
            if x4 > x1 && x3 < x2 && y4 > y1 && y3 < y2 {
                found = true;
                break;
            }
        }
        if !found {
            println!("Found max size without infinite area: {} between ({}, {}) and ({}, {})", size, x1, y1, x2, y2);
            return;
        }
    }
}


fn main() {
    let _input_file = "input.txt";
    let _input_file = "input_test.txt";
    // _part1(_input_file);
    part2(_input_file);
}