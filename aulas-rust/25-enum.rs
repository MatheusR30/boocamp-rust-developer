// ---------------------Enum & comparação com Match
enum Tipo {
    Juridica,
    Fisica,
}

struct Pessoa {
    nome: String,
    documento: String,
    tipo: Tipo,
}

fn main() {
    // let fernando = Pessoa {
    //     nome: "Tarcio".to_string(), // O que o To_String faz
    //     documento: String::from("23232333222323"), // é a mesma coisa que isso
    //     tipo: Tipo::Fisica
    // }

    let daniel = Pessoa {
        nome: String::from("Daniel"),
        documento: String::from("33.323.332/9999-99"),
        tipo: Tipo::Juridica,
    };

    // Pattern Matching
    match daniel.tipo {
        Tipo::Fisica => {
            println!("{} é uma pessoa física", daniel.nome)
        }
        _ => {
            println!("{} é uma pessoa jurídica", daniel.nome)
        }
    }
}

// ---------------------Enum & comparação com if

// #[derive(PartialEq)]
// enum Tipo {
//     Juridica,
//     Fisica,
// }

// struct Pessoa {
//     nome: String,
//     documento: String,
//     tipo: Tipo,
// }

// fn main() {
//     let daniel = Pessoa {
//         nome: String::from("Daniel"),
//         documento: String::from("323.332.879-99"),
//         tipo: Tipo::Fisica,
//     };

//     if daniel.tipo == Tipo::Fisica {
//         println!("{} é uma pessoa fisica", daniel.nome);
//     } else {
//         println!("{} é uma pessoa Juridica", daniel.nome);
//     }
// }
