
use std::io;

fn main() {

    println!("Digite o valor da tabuada");
 
    let mut valor_tabuada = String::new();
    io::stdin()
    .read_line(&mut valor_tabuada)
    .expect("Falha ao ler a linha");

    let valor_tabuada:i32 = valor_tabuada.trim()
    .parse()
    .expect("Por favor, digite um numero!");

    for multiplicador in 1..=10 {
        println!("{} X {} = {}", multiplicador, valor_tabuada,  (multiplicador * valor_tabuada));
    }

}