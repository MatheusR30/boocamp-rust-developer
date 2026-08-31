// fn main() {
//     // Criando uma tupla com tres elementos de tipos diferentes
//     let tupla: (i32, f64, u8) = (500, 6.4, 1);

//     // Acessando os elementos da tupla
//     let (x, y, z) = tupla;

//     println!("O valor de x é: {}", x);
//     println!("O valor de y é: {}", y);
//     println!("O valor de z é: {} \n", z);

//     // Acessando diretamente os elementos da tupla
//     println!("O primeiro valor é: {}", tupla.0);
//     println!("O primeiro valor é: {}", tupla.1);
//     println!("O primeiro valor é: {} \n", tupla.2);
// }

// -----------------

// fn calcular_dimensoes() -> (i32, i32) {
//     // Suponha que esses valores foram calculados
//     let largura = 30;
//     let altura = 50;
//     (largura, altura) // Retornando uma tupla
// }

// fn main() {
//     let dimensoes = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", dimensoes.0, dimensoes.1);

//     let (largura, altura) = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", largura, altura);
// }

// --------------TuplaAninhada
//

// -------------TuplasComAgumento
fn soma_dimensoes(dimensao: (i32, i32)) -> i32 {
    dimensao.0 + dimensao.1
}

fn main() {
    let dimensao = (5, 10);
    let soma = soma_dimensoes(dimensao);
    println!("Soma das dimensoes: {}", soma);
}
