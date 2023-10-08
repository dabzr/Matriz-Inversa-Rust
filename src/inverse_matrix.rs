use matrix_calc::adj;
use matrix_calc::det;
use std::vec::Vec;

pub fn inverse_matrix(matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    adj(&matrix)
        .iter()
        .map(|row| {
            row.iter()
                .map(|&element| element * 1_f32 / det(&matrix))
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>()
}
