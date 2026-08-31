// Implemente um programa que receba dois números inteiros positivos:
// o saldo disponível na conta e o valor da compra desejada. O programa
// deve verificar se o saldo é suficiente para cobrir a compra. Caso seja,
// exiba a mensagem "Compra aprovada". Caso contrário, exiba "Saldo insuficiente".
// Considere que não há taxas ou descontos, e que o valor da compra nunca será negativo.
// O saldo pode ser zero.

use std::io;

fn verificar_compra(saldo: u32, valor_compra: u32) {
    if saldo >= valor_compra {
        println!("Compra Aprovada");
    } else {
        println!("Saldo insuficiente");
    }
}
fn main() {
    // le a linha de entrada
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada");

    // Separa os valores pelo espaco
    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|valor| valor.trim().parse().ok())
        .collect();

    // Atribuiu os valores as variaveis
    let saldo = valores[0];
    let valor_compra = valores[1];

    // Verifica a compra
    verificar_compra(saldo, valor_compra);
}
