fn main() {
    let x: String = String::from("Depurando código");
    let x_modificada = mostra_na_tela_alterando(x);
    println!("=======================\n");
    println!("Olaaa !!! - {}\n", x_modificada);
    println!("=======================");
}

fn mostra_na_tela_alterando(mut str: String) -> String {
    str += " - alterando ...";
    str
}
