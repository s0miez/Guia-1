use std::io;
fn main() {
    let mut edad:String = String::new();
    println!("Cuántos años tienes?");
    let stdin = io::stdin();
    stdin.read_line(&mut edad).unwrap();
    let edad: u8 = edad.trim().parse().unwrap();
    if edad >= 18 {
        println!("Felicidades, Puedes votar!");
    } else {
        println!("Lamentablemente aún no puedes votar :(");
    }
}
