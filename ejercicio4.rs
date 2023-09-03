use std::io;
fn main() {
    let mut numero:String = String::new();
    println!("Ingresa un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut numero).unwrap();
    let numero: u8 = numero.trim().parse().unwrap();
    if numero % 2 == 0 {
        println!("{} es par!", numero);
    } else {
        println!("{} es impar!", numero);
    }
}
