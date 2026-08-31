use std::io;

fn main() {
    // let boleano = true;
    // while boleano
    loop {
        println!("Digite uma das opções abaixo: ");
        // Formatado
        println!(r#"
        Opção 1:
        Opção 2:
        Opção 3:
        Opção 4:
        "#);

        let mut opcao = String::new();
        io::stdin()
        .read_line(&mut opcao)
        .expect("Falha ao ler a linha");

    let opcao:i8 = opcao.trim().parse().expect("Por favor, digite um número.");
    

    match opcao {
        1 => println!("Você escolher a opcao Um"),
        2 => println!("Você escolher a opcao Dois"),
        3 => println!("Você escolher a opcao Tres"),
        4 => break,
        _ => println!("A opcao que você ecolheu é inválida"),
    } 
    }
}