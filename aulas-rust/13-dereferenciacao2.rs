// fn main() {
//     //memoria stack (variaveis do tipo copy no rust)
//     let mut x: i32 = 4;

//     imprime_valor(&x);
//     imprime_valor(&x);
// }

// fn imprime_valor(valor: &i32) {
//     // valor += 1; // não pode porque tenho imudabilidade nas referencias
//     println!("Valor: {}, Endereco de memoria: {:p}", valor, valor);
// }

fn main() {
    let mut x: i32 = 4;
    println!(
        "[Original] - Valor de x original: {} - referência: {:p}",
        x, &x
    );

    imprime_valor(&mut x); // Passando uma referencia mutavel para x

    println!(
        "[Original] - Valor de x apos as modificações: {} - referência: {:p}",
        x, &x
    );

    imprime_valor(&mut x); // Passando uma referencia mutavel para x

    println!(
        "[Original] - Valor de x apos as modificações: {} - referência: {:p}",
        x, &x
    );
}

fn imprime_valor(valor: &mut i32) {
    *valor += 1; // Modificando o valor referenciado por valor atualizando um reborrowing
    // O compilador pode mover a variavel temporariamente para uma localizacao diferente na memoria durante a referencia mutavel.
    // O objetivo é evitar possiveis poroblemas de aliasing e garantir a segurança das referencias
    println!(
        "[Reborrowing] - Valor referenciado por valor: {} - referencia: {:p}",
        valor, &valor
    );
}
