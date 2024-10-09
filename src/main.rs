use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    
    // Read from stdin (basically gets what was piped)
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let mut char_count = std::collections::HashMap::new();

    for c in input.chars() {
        let key = match c {
            ' ' => "space".to_string(),
            '\n' => "newline".to_string(),
            _ => c.to_string(),
        };
        *char_count.entry(key).or_insert(0) += 1;
    }

    for (c, count) in &char_count {
        println!("{}: {}", c, count);
    }
}