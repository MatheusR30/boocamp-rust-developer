#[derive(Debug)] // Permite imprimir a struct usando {:?} e {:#?}
struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
    salario: f32,
}

// Implementação dos métodos da struct Cliente
impl Cliente {
    // Verifica se o CPF foi informado.
    fn cpf_valido(&self) -> bool {
        if self.cpf.is_empty() {
            return false;
        }

        true
    }
    // Altera o campo "nome", adicionando um sobrenome ao cliente.
    fn adiciona_sobrenome(&mut self) {
        self.nome += " da Silva"
    }

    // Acrescenta um valor ao salário atual do cliente.
    fn aumento(&mut self, valor: f32) {
        self.salario += valor;
    }
}

fn main() {
    let mut cliente = Cliente {
        id: 1,
        nome: String::from("Leandro"),
        cpf: String::from("222.555.443-00"),
        salario: 5000.0,
    };

    // Adiciona "da Silva" ao nome.
    cliente.adiciona_sobrenome();

    // Concede um aumento de R$ 1.000,00.
    cliente.aumento(1000.0);

    // Executa a validação do CPF e converte o resultado
    // booleano (true/false) para um texto mais amigável.
    let valido = if cliente.cpf_valido() {
        "Verdadeiro"
    } else {
        "Falso"
    };

    // Exibe algumas informações do cliente.
    // {:#?} imprime toda a struct formatada para facilitar a leitura.
    println!(
        "O CPF do cliente({}): {} é {} \n{:#?}",
        cliente.id, cliente.nome, valido, cliente
    );
}
