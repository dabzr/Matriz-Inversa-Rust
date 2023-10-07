use matrix_calc::set_adjoint;
use matrix_calc::det;
use std::vec::Vec;

pub fn inverse_matrix(matrix: Vec<Vec<f32>>, dest: &mut Vec<Vec<f32>>, size: usize) {
    let mut adjoint: Vec<Vec<f32>> = vec![vec![0.0; size]; size];
    *dest = vec![vec![0.0; size]; size];
    let cloned_matrix = matrix.clone();
    set_adjoint(cloned_matrix, &mut adjoint, size);
    let reverse_det: f32 = 1.0 / det(&matrix, size);
    for i in 0..size {
        for j in 0..size {
            dest[i][j] = adjoint[i][j] * reverse_det;
        }
    }
}


