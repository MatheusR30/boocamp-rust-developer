// fn main() {
//     // Convertendo &str em String
//     let s1: &str = "Olá, mundo!";
//     let reference_to_s1: String = format!("{}", s1);

//     println!("s1 (referencia &str): {} - referencia: {:p}", s1, s1);
//     println!(
//         "referencia_to_s1 (String): {} - referencia: {:p}",
//         reference_to_s1, &reference_to_s1
//     );
// }

// -------------------------------

// fn main() {
//     let mut s = String::from("Ola");

//     s = s + ", mundo";
//     // s += ", mundo";
//     // s.push_str(", mundo"); // push_str() aducuiba um literal à String

//     println!("{}", s); //Isso vai exibir "Ola mundo"
// }

// ------------------------

// fn main() {
//     let mut x = 5;
//     manda_referencia(&mut x);
// }

// fn manda_referencia(x: &mut i32) {
//     *x += 1;
//     println!("{}", x)
// }

// -------------------------

// fn main() {
//     let mut x: i32 = 5;
//     x = manda_via_copia(x);

//     println!("{}", x)
// }

// fn manda_via_copia(x: i32) -> i32 {
//     x + 1
// }

// ---------------TamanhoString

fn main() {
    let s1 = String::from("texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
    // s.push_str("sss");
}
