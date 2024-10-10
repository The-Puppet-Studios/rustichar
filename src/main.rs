use std::io::{self, Read};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) {
        let mut input = Vec::<u8>::new();
        io::stdin().read_to_end(&mut input).expect("Failed to read from stdin");
    
        let mut byte_count = [0; 256];
    
        for byte in input {
            byte_count[byte as usize] += 1;
        }
    
        for (byte, count) in byte_count.iter().enumerate() {
            if *count > 0 {
                println!("{:02x}: {}", byte, count);
            }
        }
    } else {
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
}