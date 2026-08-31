// ------------DiferentesFormasDeString
// fn main() {
//     let s = "Olaa".to_string();
//     // let s = String::from("Olaá");
//     // let s = String::new(); // String enpt
//     // let s = "".to_string();

//     println!("{}", s);
// }

// -------------RemoveEspaços
// fn main() {
//     let s = "   Hello Word! ".trim();
//     // let s = "   Hello Word! ".trim_start();
//     // let s = "   Hello Word! ".trim_end();
//     println!("{}", s);
// }

// ----------------LetrasMaiusculas
// fn main() {
//     let s = "hello".to_uppercase();
//     println!("{}", s);
// }

// ----------------LetrasMinusculas
// fn main() {
//     let s = "HELLO WORLD".to_lowercase();
//     println!("{}", s);
// }

// ----------------AlterarConteudo
// fn main() {
//     let original = "Hello, world!";
//     let replaced = original.replace("world", "Rust");
//     println!("{}", replaced); // Saída: Hello, Rust

//     let original = String::from("Hello, world");
//     let replaced = original.replace("world", "Rust");
//     println!("{}", replaced); // Saída: Hello, Rust
// }

// ----------------Nomenclaturas
// use inflector::Inflector;

// fn main() {
//     let s = "hello_world".to_camel_case();
//     println!("{}", s);

//     let s = "helloWorld".to_snake_case();
//     println!("{}", s);

//     let s = "hello_world".to_pascal_case();
//     println!("{}", s);
// }

// --------------Inverter
// fn main() {
//     // Inverter caracteres de uma string
//     let s: String = "hello".chars().rev().collect();
//     println!("{}", s);
// }

// ---------------SeContemUmaPalavra
// fn main() {
//     let contains_substring = "hello, world".contains("world");
//     println!("{}", contains_substring);

//     let contains_substring = String::from("hello, world").contains("worlds");
//     println!("{}", contains_substring);
// }

// ---------------SeparandoPorEspaços
// fn main() {
//     let texto = "Hello, world! Welcome to Rust programing.";

//     // Dividindo a String pelo espaço
//     let palavras: Vec<&str> = texto.split(' ').collect();

//     println!("{:?}", palavras);
// }

// -----------PedaçoDeString
// fn main() {
//     // Pegando pecaço da String, escolhendo de 0 ate o 2
//     let s = "hello";
//     let substring = &s[0..2]; //"he"

//     println!("{}", substring);
// }

// --------------RegularExpression
// use regex::Regex;

// fn main() {
//     let email_regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap();
//     let email = "exemple@example.com";

//     if email_regex.is_match(email) {
//         println!("{} é um email válido.", email);
//     } else {
//         println!("{} é um email inválido.", email);
//     }
// }

// -------------RegrasParaPegarNumeros
use regex::Regex;
fn main() {
    let phone_regex = Regex::new(r"\(?\b\d{2}\)?\s?\d{4,5}-?\d{4}\b").unwrap();
    let text = "O meu telefone é (12) 93333-6666.";

    match phone_regex.captures(text) {
        Some(caps) => println!("Número encontrado: {}", caps.get(0).unwrap().as_str()),
        None => println!("Não foi encontrado número."),
    }
}
