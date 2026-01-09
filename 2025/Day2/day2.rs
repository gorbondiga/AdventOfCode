use std::fs;

fn is_repeated(s: &str) -> bool {
    let len = s.len();
    match len {
        l if len == 2 => return s[0..1] == s[1..2],
        l if len % 2 == 0 => {
            if (s[0..2].repeat(l / 2) == s) ||
               (s[0..(l / 2)].repeat(2) == s) ||
               (s[0..(l / 4)].repeat(4) == s) {
                return true;
            } else if (l % 5 == 0) && (l > 5) {
                return s[0..5].repeat(l / 5) == s;
            } else {
                return false;
            }
        },
        l if len % 3 == 0 && len > 3 => {
            return s[0..3].repeat(l / 3) == s;
        },
        l if len == 1 => return false,
        _ => return s[0..1].repeat(len) == s,
    }
}

fn _is_repeated(s: &str) -> bool {
    // Elegant solution proposed in reddit forum,
    // it's based on the integer log base 10 
    // to get the number lenght
    let i: usize = s.parse().expect("Invalid number");
    match 1 + i.ilog10() {
        1 => false,
        2 => i.is_multiple_of(11),
        3 => i.is_multiple_of(111),
        4 => i.is_multiple_of(101),
        5 => i.is_multiple_of(11111),
        6 => i.is_multiple_of(1001) || i.is_multiple_of(10101),
        7 => i.is_multiple_of(1111111),
        8 => i.is_multiple_of(1010101) || i.is_multiple_of(10001),
        9 => i.is_multiple_of(1001001),
        10 => i.is_multiple_of(101010101) || i.is_multiple_of(100001),
        _ => panic!(),
    }
}

fn main() {
    let mut result: i64 = 0;
    let _input_file = "input.txt";
    let contents = fs::read_to_string(_input_file)
        .expect("Failed to read input file");
    
    for range in contents.trim().split(',') { 
        let parts: Vec<&str> = range.trim().split('-').collect();
        let start: i64 = parts[0].parse().expect("Invalid number");
        let end: i64 = parts[1].parse().expect("Invalid number");
        
        for i in start..=end {
            let s = i.to_string();
            if is_repeated(&s) {
                result += i;
            }
        }
    }
    println!("======== Final result ========");
    println!("Result: {}", result);
    // wrong answer: 4174379265
    // answer: 46666175279
}