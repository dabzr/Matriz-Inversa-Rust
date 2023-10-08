mod matrix_calc;
mod inverse_matrix;
mod matrix_visual;

use matrix_calc::*;
use inverse_matrix::*;
use matrix_visual::*;

use std::io;
use std::vec::Vec;

fn main() {
    println!("Digite o fator da matriz: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let input = input.trim();
    let size: Result<usize, _> = input.parse();

    match size {
        Ok(size) => {
            let matriz: Vec<Vec<f32>> = create_random_matrix(size);
            print_matrix(matriz.clone());
            println!("O determinante da matriz é {}", det(&matriz));
            println!("A matriz adjunta é: ");
            print_matrix(adj(&matriz));
            println!("A matriz inversa é: ");
            print_matrix(inverse_matrix(&matriz));
        }
        Err(_) => {
            println!("Entrada inválida. Certifique-se de digitar um número inteiro válido.");
        }
    }
}
