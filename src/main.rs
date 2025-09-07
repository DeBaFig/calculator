pub mod calc1;
pub mod calc2;
use calc1::{add, sub};
use calc2::{multiply, rate};


mod saudacoes {
    pub fn ola() -> String {
        "Olá, mundo!".to_string()
    }
}

fn main() {

    println!("{}", saudacoes::ola());
    saudacoes::ola();

    println!("Testando as funções de cálculo:");
    let a: u32 = 10;
    let b: u32 = 5;
    println!("Adição: {}", add(a, b));
    println!("Subtração: {}", sub(a, b));
    println!("Multiplicação: {}", multiply(a, b));
    println!("Divisão: {}", rate(a, b));
    println!("Fim dos testes das funções de cálculo:");
}
