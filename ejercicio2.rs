use std::io;

fn main() {
    let mut nombre = String::new();
    println!("Cuál es tu nombre?");
    let stdin = io::stdin();
    stdin.read_line(&mut nombre).unwrap();
    println!("Tu nombre es: {}", nombre);
}
