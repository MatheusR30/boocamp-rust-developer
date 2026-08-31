// ------------ArrayPrimitivo
// fn main() {
//     // Declara um array mutavel de 3 inteiros.
//     let mut valores: [i32; 3] = [10, 20, 30];

//     // Modifica o segundo elemento do  array.
//     valores[1] = 25;

//     println!("O valor do indice é: {}", valores[0]);
//     println!("Quantidade de valores do array: {}", valores.len());

//     // Itera sobre cada elemento do array 'numeros'
//     for n in valores.iter() {
//         println!("{}", n);
//     }
// }

// -------------Array
fn main() {
    // Cria um vetor vazio de inteiros e adiciona elementos a ele
    let mut vetor: Vec<i32> = Vec::new();
    vetor.push(10);
    vetor.push(20);
    vetor.push(20);
    vetor.push(20);
    vetor.push(21);

    println!("Quantidade de valores do array: {}", vetor.len());

    for n in vetor.iter() {
        println!("{}", n);
    }

    let valor: Option<i32> = vetor.pop();

    if let Some(numero) = valor {
        println!("O valor de pop: {}", numero);
    }

    println!("Quantidade de valores do array: {}", vetor.len());

    for n in vetor.iter() {
        println!("{}", n);
    }
}
