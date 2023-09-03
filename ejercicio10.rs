use std::io;
fn main() {
    let mut num1:String = String::new();
    println!("Ingresa un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num1).unwrap();
    let num1: u8 = num1.trim().parse().unwrap();

    let mut num2:String = String::new();
    println!("Ingresa otro número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num2).unwrap();
    let num2: u8 = num2.trim().parse().unwrap();

    let mut num3:String = String::new();
    println!("Ingresa otro número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num3).unwrap();
    let num3: u8 = num3.trim().parse().unwrap();

    let mut num4:String = String::new();
    println!("Ingresa otro número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num4).unwrap();
    let num4: u8 = num4.trim().parse().unwrap();

    let suma: u8 = num1 + num2 + num3 + num4;
    let promedio: u8 = suma / 4;
    println!("El promedio de los números ingresados es: {}", promedio);
}
