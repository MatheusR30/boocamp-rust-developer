//      Exemplo de cópia

// fn main() {
//     //memoria stack (variaveis do tipo copy no rust)
//     let x: i32 = 4;
//     let y: i32 = x; //copia de dados

//     println!("O valor de x é: {} - Referencia {:p}", x, &x);
//     println!("O valor de y é: {} - Referencia {:p}", y, &y);
// }

// -----------------------------------------------------------------

//      Exemplo de Referencia

// fn main() {
//     let x: i32 = 4; // owner
//     let y: &i32 = &x; // Referencia de dados(y aposta para o mesmo local que o x)

//     println!("O valor de x é: {} - Referencia {:p}", x, &x);
//     println!("O valor de y é: {} - Referencia {:p}", y, y);
// }

// -----------------------------------------------------------------

fn main() {
    let x: i32 = 4;
    let y: i32 = x;

    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);

    // Imprimindo os endereços de memoria
    println!("Endereço de memória de x: {:p}", &x);
    println!("Endereço de memória de y: {:p}", &y);
}
