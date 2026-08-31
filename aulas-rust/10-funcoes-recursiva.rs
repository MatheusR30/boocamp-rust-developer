// fn main() {
//     mostra_na_tela(1);

// }

// fn mostra_na_tela(i: i8) {
//     if i > 10 {
//         return
//     }
//     println!("O valor de i é: {}", i);
//     mostra_na_tela(i + 1);
// }

fn main() {
    let r:i32 = mostra_na_tela(1);

    println!("O valor somado é {}", r)
}

fn mostra_na_tela(i: i32) -> i32 {
    if i > 10 {
        return i
    }

    println!("O valor de i é: {}", i);
    mostra_na_tela(i + 1)
}
