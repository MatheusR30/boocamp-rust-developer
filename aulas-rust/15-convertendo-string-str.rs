// -----------------------FORMAT-----------------------------

// fn main() {
//     // Exemplo com String
//     let mut s1 = String::from("Olá, Mundo"); // s1 é uma String alocada na memoria Heap
//     s1 += " - teste";

//     let s2 = s1.clone(); // Clonada a String s1 para s2

//     println!("String s1: {} - referencia: {:p}", s1, &s1);
//     println!("String s2: {} - referencia: {:p}", s2, &s2);

//     // Exemplo com &str
//     let s3 = "Olá mundo"; // s3 é um &str (slice de string)
//     // s3 += "- teste"; // Por ela ser por referencia, e referencia nao muda variavel

//     let s4 = format!("{} - teste", s3); // Criando um novo &str concatenando

//     // let s5 = s4; // Altera a posse - Bowrring

//     println!("&str s3: {} - referencia {:p}", s3, s3);
//     println!("&str s4: {} - referencia {:p}", s4, &s4);
// }

// ----------------------SUBSTRING---------------------------

// fn main() {
//     let original_string = String::from("Rust é incrivel");

//     // Criando uma substring usando slicing
//     let substring = &original_string[0..4];

//     println!(
//         "String original: {} - referencia: {:p}",
//         original_string, &original_string
//     );
//     println!("substring: {} - referencia: {:p}", substring, substring);
// }

fn main() {
    // Convertendo String em &str usando as_str()
    let s1 = String::from("Olá, mundo");
    let reference_to_s1: &str = s1.as_str();

    println!("s1 (String): {} - referencia: {:p}", s1, &s1);
    println!(
        "s1 (referencia &str): {} - referencia: {:p}",
        reference_to_s1, &reference_to_s1
    );

    // Convertendo String em &str fazendo uma referencia
    let s2 = String::from("Rust é incrivel!");
    let reference_to_s2: &str = &s2;

    println!("s2 (String): {} - referencia: {:p}", s2, &s2);
    println!(
        "s2 (referencia &str): {} - referencia: {:p}",
        reference_to_s2, reference_to_s2
    );
}
