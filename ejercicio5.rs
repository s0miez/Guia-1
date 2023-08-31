fn main() {
    let mut numero: i32 = -1;
    let mut completado = false;
    while !completado {
         numero += 1;
         println!("{}", numero);
         if numero == 9 {
            break;
         }
    }
    println!()
}
