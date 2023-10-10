use std::vec::Vec;

pub fn det(matrix: &[Vec<f32>]) -> f32 {
    match matrix.len() {
        1 => matrix[0][0],
        2 => (matrix[0][0] * matrix[1][1]) - (matrix[1][0] * matrix[0][1]),
        _ => (0..matrix.len())
            .map(|j| matrix[0][j] * cofactor(matrix, 0, j))
            .sum(),
    }
}
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

pub fn adj(matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    transpose(
        &((0..matrix.len())
            .map(|i| {
                (0..matrix.len())
                    .map(|j| cofactor(matrix, i, j))
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<Vec<f32>>>()),
    )
}

fn cofactor(matrix: &[Vec<f32>], row: usize, col: usize) -> f32 {
    let sgn = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
    sgn * det(&submatrix(&matrix, row, col))
}

fn submatrix(matrix: &[Vec<f32>], row: usize, col: usize) -> Vec<Vec<f32>> {
    matrix
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != row)
        .map(|(_, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, _)| j != col)
                .map(|(_, &val)| val)
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>()
}

fn transpose(matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    (0..matrix.len())
        .map(|j| {
            (0..matrix[0].len())
                .map(|i| matrix[i][j])
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>()
}

