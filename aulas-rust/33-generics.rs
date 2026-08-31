// funcao duplicada por objetivo
// fn contar_posicoes_inteiros(array: &[i32]) -> usize {
//     array.len()
// }

// fn contar_posicoes_floats(array: &[f64]) -> usize {
//     array.len()
// }

// fn contar_posicoes_strings(array: &[&str]) -> usize {
//     array.len()
// }

// fn main() {
//     let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
//     let array_strings: [&str; 3] = ["um", "dois", "três"];

//     println!(
//         "Posições no array de inteiros: {}",
//         contar_posicoes_inteiros(&array_inteiros)
//     );

//     println!(
//         "Posições no array de floats: {}",
//         contar_posicoes_floats(&array_floats)
//     );

//     println!(
//         "Posições no array de string: {}",
//         contar_posicoes_strings(&array_strings)
//     );
// }

// ------- Resolucao do Generics
// fn contar_posicoes<T>(array: &[T]) -> usize {
//     array.len()
// }

// fn main() {
//     let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];
//     let array_floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
//     let array_strings: [&str; 3] = ["um", "dois", "três"];

//     println!(
//         "Posições no array de inteiros: {}",
//         contar_posicoes(&array_inteiros)
//     );

//     println!(
//         "Posições no array de floats: {}",
//         contar_posicoes(&array_floats)
//     );

//     println!(
//         "Posições no array de string: {}",
//         contar_posicoes(&array_strings)
//     );
// }

// ---------- Funcao duplicada por objetivo I
// fn quantidade_digitos_inteiros(i: i32) -> usize {
//     i.to_string().chars().count()
// }

// fn quantidade_digitos_float(f: f64) -> usize {
//     f.to_string().chars().count()
// }

// fn quantidade_digitos_string(s: &str) -> usize {
//     s.chars().count()
// }

// fn main() {
//     let int_val: i32 = 12345;
//     let string_val: &str = "Olá josé";
//     let float_val: f64 = 123.45;

//     println!(
//         "Quantidade de digitos no inteiro: {}",
//         quantidade_digitos_inteiros(int_val)
//     );
//     println!(
//         "Quantidade de digitos no float: {}",
//         quantidade_digitos_float(float_val)
//     );
//     println!(
//         "Quantidade de digitos no string {}",
//         quantidade_digitos_string(string_val)
//     );
// }

//--------------- Funcao duplicada por objetos II
// fn quantidade_digitos<T>(i: T) -> usize {
//     i.to_string().chars().count()  ????????
// }
// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let string_val: &str = "Olá josé";

//     println!(
//         "Quantidade de digitos no inteiro: {}",
//         quantidade_digitos(int_val)
//     );
//     println!(
//         "Quantidade de digitos no float: {}",
//         quantidade_digitos(float_val)
//     );
//     println!(
//         "Quantidade de digitos no string {}",
//         quantidade_digitos(string_val)
//     );
// }

// ------------ Resolucao do Generics I / III "Ultimo exemplo, corrigido, com o Blaba, melhor usar nosso contrato do que o Display, já que la teremos que implementar todas as possibilidade"

// trait ContaCaracteres {
//     fn conta_caracteres(&self) -> usize;
// }

// impl ContaCaracteres for i32 {
//     fn conta_caracteres(&self) -> usize {
//         self.to_string().chars().count()
//     }
// }

// impl ContaCaracteres for f64 {
//     fn conta_caracteres(&self) -> usize {
//         self.to_string().chars().count()
//     }
// }

// impl ContaCaracteres for String {
//     fn conta_caracteres(&self) -> usize {
//         self.chars().count()
//     }
// }

// impl<'a> ContaCaracteres for &'a str {
//     fn conta_caracteres(&self) -> usize {
//         self.chars().count()
//     }
// }

// fn quantidade_caracteres<T: ContaCaracteres>(valor: T) -> usize {
//     valor.conta_caracteres()
// }

// struct Blaba {
//     x: i32,
// }

// impl ContaCaracteres for Blaba {
//     fn conta_caracteres(&self) -> usize {
//         self.x.to_string().chars().count()
//     }
// }
// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let str_val: &str = "Olá josé";
//     let string_val: String = "Olá josé".to_string();

//     let blaba_val: Blaba = Blaba { x: 5 };

//     println!(
//         "Quantidade de caracteres no inteiro: {}",
//         quantidade_caracteres(int_val)
//     );
//     println!(
//         "Quantidade de caracteres no float: {}",
//         quantidade_caracteres(float_val)
//     );
//     println!(
//         "Quantidade de caracteres no string: {}",
//         quantidade_caracteres(str_val)
//     );

//     println!(
//         "Quantidade de caracteres no string: {}",
//         quantidade_caracteres(string_val)
//     );

//     println!(
//         "Quantidade de dígitos no blaba: {}",
//         quantidade_caracteres(blaba_val)
//     );
// }

// ------------ Resolucao Generics II

/*
O trait Display da biblioteca padrão pode ser utilizado para converter os tipos em uma
forma que possa ser representada como uma string. Uma vez que um tipo implemente Display.
ele pode ser convertido em String e, em seguida, podemos contar os caracteres
*/

// use std::fmt::Display; // Trait que tem uma funcao comum to_string()

// fn quantidade_caracteres<T: Display>(valor: T) -> usize {
//     valor.to_string().chars().count()
// }

// struct Blaba {
//     x: i32,
// }

// impl Display for Blaba {
//     fn to_string(&self) -> usize {
//         self.x.to_string().chars().count()
//     }
// }

// fn main() {
//     let int_val: i32 = 12345;
//     let float_val: f64 = 123.45;
//     let string_val: &str = "Olá José";
//     // let blaba_val: Blaba = Blaba { x: 5 };

//     println!(
//         "Quantidade de dígitos no inteiro: {}",
//         quantidade_caracteres(int_val)
//     );

//     println!(
//         "Quantidade de dígitos no float: {}",
//         quantidade_caracteres(float_val)
//     );

//     println!(
//         "Quantidade de dígitos no string: {}",
//         quantidade_caracteres(&string_val)
//     );

//     println!(
//         "Quantidade de dígitos no blaba: {}",
//         quantidade_caracteres(&blaba_val)
//     );
// }

// ------------- Generics basic I

// Struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let int_point: Point<i32> = Point { x: 5, y: 10};
//     let float_point: Point<f64> = Point { x: 1.0, y: 4.0};
//     let string_point: Point<&str> = Point { x: "1.0", y: "4.0"};
// }

// -------------- generics
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn retorna_valor_de_x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.retorna_valor_de_x());
// }

// --------------- Generics
// use std::fmt::Display;

// fn print<T: Display>(item: T) {
//     println!("{}", item);
// }

// fn main() {
//     print(1); // Int
//     print(String::from("hello")); // String
//     print("Hello"); // &str
//     print(1.5); // f64
// }

// ---------------- Generic Mult
// struct Pair<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Pair<T, U> {
//     fn new(x: T, y: U) -> Self {
//         Self { x, y }
//     }
// }

// fn main() {
//     let pair = Pair::new(5, 10.5);
//     println!("O valor de x: {}", pair.x);

//     let pair2 = Pair::new(5, "O valor de Y");
//     println!("O valor de y: {}", pair2.y);
// }

/*
=== Tipos de Traits ===
A trait Debug é usada para habilitar a funcionalidade de formatação de saída de debug para
Quando um tipo implementa a trait Debug, ele pode ser formatado usando o especificador
de formatação {:?} ou {:#} (para uma saída mais "bonita", também conhecida como "prett")
Isso é particularmente util durante o desenvolvimento e para debugging, pois permite
valores de uma forma legivel para o desenvolvedor.

=== [use std::cmp::PartialOrd; ] ===
A trait PartilOrd permite comparacoes parciais entre valores de um tipo o que significa
nem todos os valores podem ser comparaveis entre si. Ela fornece a funcionalidade para
verificar se um valor é menor que, igual a, ou maior que outro valor, retornando
Some(true), Some(false), ou None quando a comparação não é possivel
(por exemplo, quando comparando numeros flutuantes NaN). A trait PartialOrd é uma
supertrait da trait PartialEqm, que fornece funcionalidade para testar igualdade e desigualdade

=== { use Copy; ] ===
A Trait Copy em Rust é uma trait especial que indica que os valores do tipo em questao podem
ser duplicados simplesmente copiando seus bits, uma operacao
conhecxida como shallow copy. Isso é diferente de um deep copy, que copiaria nao aprenas
o valor em si, mas tambem qualquer dado ao qual ele se refere indiretamente.

A trait Copy é comumente implementada por tipos simples e sem alocacao de heap, como numeros
interios, pontos flutuantes, e outros tipos primiticos, assim como tuplas e arrays desses tipos,
desde que todos os elementos tambem implementem Copy.
*/

// use std::cmp::PartialOrd;
// use std::fmt::Debug;

// fn compare_and_display<T, U>(a: T, b: U)
// where
//     T: PartialOrd + Debug,
//     U: Into<T>,
// {
//     let b: T = b.into();
//     if a > b {
//         println!("{:?} is great than {:?}", a, b);
//     } else {
//         println!("{:?} is not great than {:?}", a, b);
//     }
// }

// fn main() {
//     compare_and_display(10, 5);
// }

// ---------------------Banco de dados
// trait DatabaseService {
//     fn save_message(&self, message: &str);
//     fn show_message(&self) -> String;
// }

// struct MySQLService;

// impl DatabaseService for MySQLService {
//     fn save_message(&self, message: &str) {
//         println!("Saving '{}' to MySQL", message);
//         // Aqui iria a lógica para salvar a mensagem no MySQL
//     }

//     fn show_message(&self) -> String {
//         let message = "Message from MySQL";
//         println!("Fetching message from MySQL: {}", message);
//         // Aqui iria a lógica para buscar a mensagem do MySQL
//         message.to_string()
//     }
// }

// struct PostgreSQLService;

// impl DatabaseService for PostgreSQLService {
//     fn save_message(&self, message: &str) {
//         println!("Saving '{}' to PostgreSQL", message);
//         // Aqui iria a lógica para salvar a mensagem do PostgreSQL
//     }

//     fn show_message(&self) -> String {
//         let message = "Messagem from PostgreSQL";
//         println!("Fetching message from PostgreSQL: {}", message);
//         // Aqui iria a lógica para buscar a mensagem do PostgreSQL
//         message.to_string()
//     }
// }

// struct GenericService<T: DatabaseService> {
//     database_service: T,
// }

// impl<T: DatabaseService> GenericService<T> {
//     fn new(database_service: T) -> Self {
//         GenericService { database_service }
//     }

//     fn save_message(&self, message: &str) {
//         self.database_service.save_message(message);
//     }

//     fn show_message(&self) -> String {
//         self.database_service.show_message()
//     }
// }

// fn main() {
//     let mysql_service = MySQLService;
//     let postgres_service = PostgreSQLService;

//     let mysql_generic_service = GenericService::new(mysql_service);
//     let postgres_generic_service = GenericService::new(postgres_service);

//     mysql_generic_service.save_message("Hello, World!");
//     let message_from_mysql = mysql_generic_service.show_message();
//     println!("{}", message_from_mysql);

//     postgres_generic_service.save_message("Hello, Rust!");
//     let message_from_postgres = postgres_generic_service.show_message();
//     println!("{}", message_from_postgres);
// }

// ---------------------------Serialize I
// use serde::Serialize;
// use serde_json::to_string_pretty;

// // Transformando essa derivativa em um Json
// #[derive(Serialize)]
// struct Produto {
//     id: u32,
//     nome: String,
//     preco: f64,
// }

// // Transformando essa derivativa em um Json
// #[derive(Serialize)]
// struct Cliente {
//     id: u32,
//     nome: String,
//     email: String,
// }

// // Funcao genérica para imprimir propriedades e valores de uma struct
// fn imprimir_propriedades<T: Serialize>(item: &T) {
//     let json = to_string_pretty(item).unwrap_or_else(|_| "Falha na serialização".to_string());
//     println!("{}", json);
// }

// fn main() {
//     let produto = Produto {
//         id: 1,
//         nome: "Caneta".to_string(),
//         preco: 1.50,
//     };

//     let cliente = Cliente {
//         id: 101,
//         nome: "Joao Silva".to_string(),
//         email: "joao.silva@example.com".to_string(),
//     };

//     imprimir_propriedades(&produto);
//     imprimir_propriedades(&cliente);
// }

// ---------------------------Serialize II
use serde::Serialize;
use serde_json::to_string_pretty;

// Transformando essa derivativa em um Json
#[derive(Serialize)]
struct Produto {
    id: u32,
    nome: String,
    preco: f64,
}

// Transformando essa derivativa em um Json
#[derive(Serialize)]
struct Cliente {
    id: u32,
    nome: String,
    email: String,
}

// Funcao genérica para imprimir propriedades e valores de uma struct
fn imprimir_propriedades(item: &impl Serialize) {
    // fn imprimir_propriedades (item: &dyn Serialize) {
    let json = to_string_pretty(item).unwrap_or_else(|_| "Falha na serialização".to_string());
    println!("{}", json);
}

fn main() {
    let produto = Produto {
        id: 1,
        nome: "Caneta".to_string(),
        preco: 1.50,
    };

    let cliente = Cliente {
        id: 101,
        nome: "Joao Silva".to_string(),
        email: "joao.silva@example.com".to_string(),
    };

    imprimir_propriedades(&produto);
    imprimir_propriedades(&cliente);
}

/*
=== Conclusao ===
Código 1 usa generics com traill bounds explicitamente, o que é util para quando voce quer
clareza total sobre a genereicidade e esta prparado para lidar com a verbosidade

Código 2 simplifica a assinatura da funcao usando impl Trait, tornando o código mais limpo e
facil de ler, mantenndo a eficienca do monorfizacao, se fosse usado &dyn Serialize,
introduziria polimorfismo dinamico com uma ligeira penalidade de desempenho, mas com bem de flexibilidade

*/
