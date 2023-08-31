use std::io;
fn main() {
    let mut edad:String = String::new();
    let stdin = io::stdin();
    println!("Cuántos años tienes?");
    stdin.read_line(&mut edad).unwrap();
    let edad: u8 = edad.trim().parse().unwrap();
    if edad >= 18 {
        println!("Felicidades, Puedes votar!");
    } else {
        println!("Lamentablemente aún no puedes votar :(");
    }
}
