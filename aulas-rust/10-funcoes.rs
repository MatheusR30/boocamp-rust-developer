fn main() {
    let x: i32 = 5;

    // let y = calculo();
    let y: i32 = {
        let x: i32 = 3;
        println!("O valor de x é: {}", x);
        x + 1
    };

    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
}

// fn calculo() -> i32 {
//     let x: i32 = 3;
//     println!("O valor de x é {}" , x);
//     x + 1
// }
