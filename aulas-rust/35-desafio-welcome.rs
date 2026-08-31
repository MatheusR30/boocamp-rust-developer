use std::io;

fn main() {
    // Lê uma linha da entrada padrão
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.len() == 2 {
            let nome = parts[0];
            let tipo = parts[1];
            println!("Welcome, {}! Your account type is {}.", nome, tipo);
        } else {
            println!("Invalid input.");
        }
    }
}
