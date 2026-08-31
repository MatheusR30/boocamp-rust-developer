use std::io;

fn soma_entre_valores(x: i16, y: i16) -> i16 {
    x + y
}

fn subtracao_entre_valores(x: i16, y: i16) -> i16 {
    x - y
}

fn solicita_parametros_para_calculo(soma: bool) {
    let mut x: String = String::new();
    let mut y: String = String::new();

    println!("Digite o primeiro valor");
    io::stdin()
        .read_line(&mut x)
        .expect("Falha ao ler a linha, digite um numero");

    println!("Digite o segundo valor");
    io::stdin()
        .read_line(&mut y)
        .expect("Falha ao ler a linha, digite um numero");

    let x: i16 = x.trim().parse().expect("Por favor, digite um número.");
    let y: i16 = y.trim().parse().expect("Por favor, digite um número.");

    let r = if soma {
        soma_entre_valores(x, y)
    } else {
        subtracao_entre_valores(x, y)
    };

    println!("O resultado entre os valores é de: {}", r);
}

fn solicita_tabuada() {
    println!("Digite o valor da tabuada");

    let mut valor_tabuada = String::new();
    io::stdin()
        .read_line(&mut valor_tabuada)
        .expect("Falha ao ler a linha");

    let valor_tabuada: i32 = valor_tabuada
        .trim()
        .parse()
        .expect("Por favor, digite um numero!");

    for multiplicador in 1..=10 {
        println!(
            "{} X {} = {}",
            multiplicador,
            valor_tabuada,
            (multiplicador * valor_tabuada)
        );
    }
}

fn menu() {
    loop {
        println!("Digite uma das opções abaixo: ");
        println!(
            r#"
         1) Soma entre valores
         2) Subtração entre valores
         3) Criar a tabuada de um número
         0) Encerrar o programa
        "#
        );

        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a linha");

        let opcao: i16 = opcao.trim().parse().expect("Por favor, digite um número.");

        match opcao {
            1 => solicita_parametros_para_calculo(true),
            2 => solicita_parametros_para_calculo(false),
            3 => solicita_tabuada(),
            0 => break,
            _ => println!("A opcao que você ecolheu é inválida"),
        }
    }
}

fn main() {
    menu();
}
