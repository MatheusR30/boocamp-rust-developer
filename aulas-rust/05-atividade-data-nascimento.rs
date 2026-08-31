/*
Dado que eu tenha um ano de nascimento, e faço a subtracao 
pelo ano atual, entao devo ter o valor da idade da pessoa 
*/
fn main() {
    let nome:&str = "Matheus";

    let ano_nascimento: u16 = 1994;
    let mes_nascimento: u16 = 1;
    let dia_nascimento: u16 = 31;
    
    let ano_atual: u16 = 2026;
    let mes_atual:u16 = 1;
    let dia_atual:u16 = 30;

    let mut idade: u16 = ano_atual - ano_nascimento;

    if mes_nascimento > mes_atual {
        idade -= 1;
    } else if dia_nascimento > dia_atual {
        idade -= 1;
    }

    println!("A idade do(a) {} calculada para o ano de {} é de {} anos", nome, ano_nascimento, idade);

}