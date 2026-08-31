struct Funcionario {
    id: i32,
    nome: String,
    salario: f32,
}

fn main() {
    // Matriz
    let lista_de_funcionarios: [[String; 3]; 2] = [
        [
            String::from("1"),
            String::from("Matheus"),
            String::from("1200"),
        ],
        [
            String::from("2"),
            String::from("Romario"),
            String::from("1700"),
        ],
    ];

    println!("Funcionario 1");
    println!("Id: {}", lista_de_funcionarios[0][0]);
    println!("nome: {}", lista_de_funcionarios[0][1]);
    println!("Salario: {}", lista_de_funcionarios[0][2]);

    println!("{}", "-".repeat(20)); //Repete 20x o traço

    println!("Funcionario 2");
    println!("Id: {}", lista_de_funcionarios[1][0]);
    println!("nome: {}", lista_de_funcionarios[1][1]);
    println!("Salario: {}", lista_de_funcionarios[1][2]);

    println!("{}", "-".repeat(20)); //Repete 20x o traço

    // Array
    let fun = [
        String::from("1"),
        String::from("Array"),
        String::from("1300,0"),
    ];
    println!("Id: {}", fun[0]);
    println!("Nome: {}", fun[1]);
    println!("Salario: {}", fun[2]);

    println!("{}", "-".repeat(20)); //Repete 20x o traço

    // tupla
    let func: (i32, String, f32) = (1, String::from("Tupla"), 1300.0);

    println!("Id: {}", func.0);
    println!("Nome: {}", func.1);
    println!("Salario: {}", func.2);

    println!("{}", "-".repeat(20)); //Repete 20x o traço

    // Structs
    let funcionario = Funcionario {
        id: 1,
        nome: String::from("Structs"),
        salario: 1300.0,
    };

    println!("Id: {}", funcionario.id);
    println!("Nome: {}", funcionario.nome);
    println!("Salario: {}", funcionario.salario);
}
