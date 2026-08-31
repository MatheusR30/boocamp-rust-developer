// ------------------ Solucao ---------------------
/*
// 'a = uma anotação de lifetime (tempo de vida)
São usads paga garantir que referências a dados não outlive (sovrevivam mais que) aos dados que aos quais elas apontam.

Elas são umapartedunfamental do ssitema de tipos de Rust,
permitindo que ocompilador verifica em tempo de compilacao que os dados referenciados nao serao desalocados enquanto ainda existirem referencias a eles, evitando assim dangling referebces *referencias penduradas) e garantindo seguranda de memória.
*/

// fn quem_e_maior<'a>(x: &'a str, y: &'a str) -> &'a str {
//     println!("Endereço de memória de x: {:?}", x.as_ptr());

//     println!("Endereço de memória de y: {:?}", y.as_ptr());

//     if x.chars().count() > y.chars().count() {
//         x // retornando o ganhador
//     } else {
//         y // retornando o ganhador
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     println!("Endereço de memória da string: {:?}", string1.as_ptr());

//     let string2 = "xyz";
//     println!("Endereco de memoria de string2: {:?}", string2.as_ptr());

//     let result = quem_e_maior(string1.as_str(), string2);
//     println!("A maior string é {}", result);
//     println!(
//         "Endereço de memória de result o ganhador: {:?}",
//         result.as_ptr()
//     );
// }

// -------------------- Erro ---------------------

// fn quem_e_maior(x: &str, y: &str) -> &str {
//     println!("Endereço de memória de x: {:?}", x.as_ptr());
//     println!("Endereço de memória de y: {:?}", y.as_ptr());

//     if x.len() > y.len() {
//         x // retornando o ganhador
//     } else {
//         y // retornando o ganhador
//     }
// }

fn main() {
    let string1 = String::from("abcd");
    println!("Endereço de memória da string: {:?}", string1.as_ptr());

    let string2 = "xyz";
    println!("Endereco de memoria de string2: {:?}", string2.as_ptr());

    // let result = quem_e_maior(string1.as_str(), string2);
    //     println!("A maior string é {}", result);
    //     println!(
    //         "Endereço de memória de result o ganhador: {:?}",
    //         result.as_ptr()
    //     );
}
