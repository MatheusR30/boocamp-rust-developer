
fn main() {
    println!("{}",retorna_string(11));
}

fn retorna_string(param:i32) -> String {
    if param == 10 {
        return String::from("Este numero é igual a 10");
    }
    String::from("teste")
}