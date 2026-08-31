mod enums;
mod models;

use enums::tipo::Tipo;
use models::pessoa::Pessoa;
use modulo_app_2::mostra_oi;

fn main() {
    let daniel = Pessoa::new("Daniel", "222.222.222.33", Tipo::Fisica);

    daniel.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha divisória

    let c_e_c = Pessoa::new("C&C", "222.222.0000-33", Tipo::Juridica);

    c_e_c.show();

    println!("{}", "-".to_string().repeat(20)); // Imprime uma linha
    mostra_oi()
}
