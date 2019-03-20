use std::io::{self, Write};

fn main() {
    print!("Hello, what's your name?\n> ");
    io::stdout().flush().expect("Failed to flush stdout buffer");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    println!("Nice to meet you, {}.", name.trim());
}
