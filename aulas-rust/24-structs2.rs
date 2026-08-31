struct Funcionario {
    id: i32,
    nome: String,
    salario: f32,
}

// struct FuncionarioNull {
//     id: Option<i32>,
//     nome: Option<String>,
//     salario: Option<i32>,
// }

fn main() {
    let funcionario: Funcionario = Funcionario {
        id: 1,
        nome: String::from("Danilo"),
        salario: 1300.0,
    };

    let funcionario2: Funcionario = Funcionario {
        nome: String::from("Matheus"),
        ..funcionario
    };
    println!("Id: {}", funcionario2.id);
    println!("Nome: {}", funcionario2.nome);
    println!("Salario: {}", funcionario2.salario);

    // let mut funcionario = build_funcionario(String::from("Danilo"), 2400.0);

    // let mut funcionario: Funcionario = Funcionario {
    //     id: 0,
    //     nome: String::new(),
    //     salario: 0.0,
    // };

    // let mut funcionario = construir_funcionario_null();
    // let mut funcionario = construir_funcionario();

    // funcionario.id = Some(5);
    // funcionario.nome = Some(String::from("Fabricio"));

    // println!("Id: {}", funcionario.id.unwrap_or(0));
    // println!(
    //     "Nome: {}",
    //     funcionario
    //         .nome
    //         .as_ref()
    //         .unwrap_or(&String::from("Desconhecido"))
    // );
    // println!("Salario: {}", funcionario.salario.unwrap_or(0));
}

// fn build_funcionario(nome: String, salario: f32) -> Funcionario {
//     Funcionario {
//         id: 0,
//         nome: nome,
//         salario: salario,
//     }
// }

// fn construir_funcionario_null() -> FuncionarioNull {
//     FuncionarioNull {
//         id: None,
//         nome: None,
//         salario: None,
//     }
// }
