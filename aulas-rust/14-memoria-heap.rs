// fn main() {
// variaveis na memóirio HEAP

// não é uma variavel by copy
// let s1 = String::from("Olá"); // s1 possui a propriedade da String
// let s2 = s1; // A propriedade é transferida de s1 para s2(Borrowing)

// Isso causa um erro, porque s1 não é mais valido apos a transferencia
// println!("s1: {} - referencia: {:p}", s1, &s1);

// s2 é válido e pode ser usado
//     println!("s2: {} - referencia {:p}", s2, &s2);
// }

// ----------------------------------CLONE-------------------------------------------------

fn main() {
    // variaveis na memóirio HEAP

    let s1 = String::from("Olá"); // s1 possui a propriedade da String
    let s2 = s1.clone(); // s2 recebe uma copia

    println!("s1: {} - referencia: {:p}", s1, &s1);

    // s2 é válido e pode ser usado
    println!("s2: {} - referencia {:p}", s2, &s2);
}
