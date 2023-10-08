mod matrix_calc;
mod inverse_matrix;
mod matrix_visual;

use std::io;
use std::vec::Vec;
use matrix_visual::print_matrix;

fn main() {
    println!("Digite o fator da matriz: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let input = input.trim();
    let size: Result<usize, _> = input.parse();

    match size {
        Ok(size) => {
            let matriz: Vec<Vec<f32>> = matrix_visual::create_random_matrix(size);
            let adjunta: Vec<Vec<f32>> = matrix_calc::adj(&matriz);
            let inversa: Vec<Vec<f32>> = inverse_matrix::inverse_matrix(&matriz);
            print_matrix(&matriz);
            println!("O determinante da matriz é {}", matrix_calc::det(&matriz));
            println!("A matriz adjunta é: ");
            print_matrix(&adjunta);
            println!("A matriz inversa é: ");
            print_matrix(&inversa);
        }
        Err(_) => {
            println!("Entrada inválida. Certifique-se de digitar um número inteiro válido.");
        }
    }

}
