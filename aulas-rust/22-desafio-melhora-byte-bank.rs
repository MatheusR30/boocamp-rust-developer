// ---------------SemConceitoChamadoDRY(Don't Repeat Yourself).

// use std::io;

// fn aprovar_compra(saldo_disponivel: i32, valor_compra: i32) {
//     if saldo_disponivel >= valor_compra {
//         println!(
//             "Compra aprovada! Saldo atual: R${}",
//             saldo_disponivel - valor_compra
//         );
//     } else {
//         println!(
//             "Saldo insuficiente. Faltam: R${}",
//             valor_compra - saldo_disponivel
//         );
//     }
// }

// fn main() {
//     // Strings vazias e mutaveis que recebem valores digitados
//     let mut input_saldo = String::new();
//     let mut input_compra = String::new();

//     // 2. Pedimos e lemos o Saldo
//     println!("Digite o seu saldo disponivel:");
//     io::stdin()
//         .read_line(&mut input_saldo)
//         .expect("Falha ao ler o saldo");

//     // 3. Pedimos e lemos o valor da compra
//     println!("Digite o valor da compra:");
//     io::stdin()
//         .read_line(&mut input_compra)
//         .expect("Falha ao ler o valo da compra");

//     // 4. Convertemos os textos em números inteiros
//     let saldo: i32 = input_saldo
//         .trim()
//         .parse()
//         .expect("Por favor digite um número válido para o saldo");

//     let compra: i32 = input_compra
//         .trim()
//         .parse()
//         .expect("por favor, digite um número válido para a compra");

//     // 5. Chamamos a sua função
//     aprovar_compra(saldo, compra);
// }

// ------------------ComConceitoChamadoDRY (Don't Repeat Yourself).
// Nao repetir códigos que fazem a mesma coisa

use std::io;

fn verificar_compra(saldo_disponivel: u32, valor_compra: u32) {
    if saldo_disponivel >= valor_compra {
        print!("Compra aprovada");
    } else {
        print!("Saldo insuficiente");
    }
}

fn ler_inteiro_positivo(mensagem: &str) -> u32 {
    println!("{}", mensagem);

    let mut entrada_usuario = String::new();

    io::stdin()
        .read_line(&mut entrada_usuario)
        .expect("Falha ao ler a entrada");

    entrada_usuario
        .trim()
        .parse()
        .expect("Por favor, digite um número inteiro positivo válido")
}

fn main() {
    // Coleta dos dados do usuario
    let saldo = ler_inteiro_positivo("Digite o saldo disponivel:");
    let compra = ler_inteiro_positivo("Digite o valor da compra desejada:");

    // 5. Chamamos a sua função
    verificar_compra(saldo, compra);
}
