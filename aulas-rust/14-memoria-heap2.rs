// fn main() {
//     // variaveis na memóirio HEAP

//     let s1 = String::from("Olá"); // s1 possui a propriedade da String
//     let s2 = s1.clone(); // A propriedade é transferida de s1 para s2(Borrowing)

//     // Isso causa um erro, porque s1 não é mais válido apos a transferencia
//     println!("Antes da transferência:");
//     print_string(&s1);
//     print_string(&s2);
// }

// fn print_string(s: &String) {
//     println!("Valor da Stringg: {} - referencia: {:p}", s, s);
// }

// // -------------------------------------- Slice -----------------------

// fn main() {
//     // Exemplo com String
//     let s1 = String::from("Olá Mundo"); // s1 é uma String alocada na memoria Heap
//     let s2 = s1.clone(); // Clonada a String s1 para s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3 = "Olá mundo"; // s3 é um &str (slice de string)
//     let s4 = s3; // s4 é uma referencia para o mesmo &str

//     println!("&str s3: {} - referencia {:p}", s3, s3);
//     println!("&str s4: {} - referencia {:p}", s4, s4);
// }

// ------------------------------------receita´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´

fn main() {
    // Exemplo com String
    let mut s1 = String::from("Olá, Mundo"); // s1 é uma String alocada na memoria Heap
    s1 += " - teste";

    let s2 = s1.clone(); // Clonada a String s1 para s2

    println!("String s1: {} - referencia: {:p}", s1, &s1);
    println!("String s2: {} - referencia: {:p}", s2, &s2);

    // Exemplo com &str
    let s3 = "Olá mundo"; // s3 é um &str (slice de string)
    // s3 += "- teste"; // Por ela ser por referencia, e referencia nao muda variavel

    let s4 = s3; // s4 é uma referencia para o mesmo &str

    println!("&str s3: {} - referencia {:p}", s3, s3);
    println!("&str s4: {} - referencia {:p}", s4, s4);
}
