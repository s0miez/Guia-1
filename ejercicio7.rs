use std::io;
fn factorial(n:i32) -> i32 {
    if n == 0 {
        return 1;
    } else {
        return n*factorial(n - 1);
    }
}

fn main() {
    let mut numero:String = String::new();}
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut numero).unwrap();
    let numero: u8 = numero.trim().parse().unwrap();
    println!("Factorial de {} es {}", numero, factorial(numero.into()))
}
