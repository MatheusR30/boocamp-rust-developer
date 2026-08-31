/*
Em Rust, "lifetimes" (tempo ed vida) são uma forma de o compilador
garantir que referencias não persistam alem da existencia dos dados
aos quais elas apontam, prevenido assim erros comuns como dagling
pointers(ponteiros para dados ja deslocados

Barrow Chaecker = "checagem de emprestimos" foi projetados para
previnir erros*/

// -----------------Erro
// fn retorna_mensagem_de_teste() -> &String {
//     let local = String::from("isso é um teste");
//     &local // tentativa de retornar uma referencia para uma String
// }

// fn main() {
//     let result = retorna_mensagem_de_teste();
//     println!("lifetime erro {}", result);
// }

// --------------- Solucao -------------------
// fn retorna_mensagem_de_teste() -> String {
//     let local = String::from("isso é um teste");
//     local // Retorna a String diretamente / uma transferencia de propriedade(ownership)
// }

// fn main() {
//     let result = retorna_mensagem_de_teste(); // a result se torna "ownership"
//     println!("lifetime error {}", result);
// }

// --------------- Solucao tempo de vida (life time)-------------------
fn retorna_mensagem_de_teste() -> String {
    let local = String::from("isso é um teste");
    local // Retorna a String diretamente / uma transferencia de propriedade(ownership)
}

fn main() {
    let result = retorna_mensagem_de_teste(); // a result se torna "ownership"
    println!("lifetime error {}", result);
}
