mod matrix_calc;
mod inverse_matrix;
mod matrix_visual;

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
            let mut matriz: Vec<Vec<f32>> = Vec::new();
            let mut adjunta: Vec<Vec<f32>> = Vec::new(); 
            let mut inversa: Vec<Vec<f32>> = Vec::new();
            matrix_visual::create_random_matrix(&mut matriz, size);
            matrix_calc::set_adjoint(matriz.clone(), &mut adjunta, size);
            matrix_visual::print_matrix(matriz.clone(), size);
            println!("O determinante da matriz é {}", matrix_calc::det(&matriz, size));
            println!("A matriz adjunta é: ");
            matrix_visual::print_matrix(adjunta, size);
            println!("A matriz inversa é: ");
            inverse_matrix::inverse_matrix(matriz, &mut inversa, size);
            matrix_visual::print_matrix(inversa, size);
        }
        Err(_) => {
            println!("Entrada inválida. Certifique-se de digitar um número inteiro válido.");
        }
    }

}
