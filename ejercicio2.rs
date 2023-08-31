use std::io;

fn main() {
    let mut nombre = String::new();
    let stdin = io::stdin();
    println!("Cuál es tu nombre?");
    stdin.read_line(&mut nombre).unwrap();
    println!("Tu nombre es: {}", nombre);
}
