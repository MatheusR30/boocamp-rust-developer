// fn main() {
//     let x: i32 = 4; // owner
//     let y: &i32 = &x; // y é uma referencia para x

//     println!("O valor de x é: {}", x);
//     println!("O valor de y é: {}", y);

//     // Imprimindo os endereços de memoria
//     println!("Endereço de memória de x: {:p}", &x); // {:p} imprime o endereço
//     println!("Endereço de memória de y: {:p}", y); // y já é uma referencia,

//     let t = y; // cria outra referencia para o dono x
//     println!("Endereço de memória de t: {:p}", t);

//     let w = *y; // Desreferência com copo para w
//     println!("O valor de w {}, Endereço de memória de w: {:p}", w, &w);
// }

// fn main() {
//     let mut x: i32 = 4; // Declare x como mutavel
//     let y: &i32 = &x; // y é uma referencia para x

//     println!("O valor de x é: {}", x);
//     println!("O valor de y é: {}", y);

// Modifique x para invalidar y
// x = 42; // Modifique o owner

// Agora, y se tornou uma referencia invalida
// Tentar imprimir y resultara em um erro de tempo de compilação

// println!("o valor de y é: {}", y);
//     println!("o valor de x é: {}", x);
// }

fn main() {
    //memoria stack (variaveis do tipo copy no rust)
    let x: i32 = 4; // Declare x como mutavel
    let y = &x; // copia de dados

    imprime_valor(&x);
    imprime_valor(y);
}

fn imprime_valor(valor: &i32) {
    println!("Endereço de memória: {:p}", valor);
}
