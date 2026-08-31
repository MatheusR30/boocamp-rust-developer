fn main() {
    let x:i32 = 5;
    println!("O valor de x e sua memória: {}, {:p}", x, &x);
    
    let x:i32 = x + 1;
    println!("O valor de x e sua memória: {}, {:p}", x, &x);
    
    let x:i32 = x * 2;
    println!("O valor de x e sua memória: {}, {:p}", x, &x);

    println!("O valor de x é: {}", x);
}