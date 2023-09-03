fn main() {
    let mut numero: i32 = 11;
    let mut completado = false;
    while !completado {
         numero -= 1;
         println!("{}", numero);
         if numero == 0 {
            break;
         }
    }
}
