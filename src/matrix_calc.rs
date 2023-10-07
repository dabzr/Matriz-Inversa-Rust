use std::vec::Vec;

pub fn set_adjoint(src: Vec<Vec<f32>>, dest: &mut Vec<Vec<f32>>, size: usize){
    *dest = vec![vec![0.0; size]; size];
    for i in 0..size {
        for j in 0..size {
            dest[i][j] = cofactor(&src, size, i, j);
        }
    }
}

fn cofactor(matrix: &Vec<Vec<f32>>, size: usize, row: usize, col:  usize) -> f32{
    let mut subi: usize = 0;
    let mut submatrix: Vec<Vec<f32>> = vec![vec![0.0; size - 1]; size - 1];
    for i in 0..size {
        if i== row {continue;}
        let mut subj: usize = 0;
        for j in 0..size {
            if j == col {continue;}
            submatrix[subi][subj] = matrix[i][j];
            subj += 1;
        }
        subi += 1;
    }
    det(&submatrix, size-1)
}

pub fn det(matrix: &Vec<Vec<f32>>, size: usize) -> f32{
    let mut det: f32 = 0f32;
    let mut sgn: i8 = 1;
    match size{
        1 => det = matrix[0][0],
        2 => det = (matrix[0][0] * matrix[1][1]) - (matrix[1][0] * matrix[0][1]),
        _ => 
            for j in 0..size {
                det += sgn as f32 * matrix[0][j] * (cofactor(&matrix, size, 0, j));
                sgn = -sgn;
            },
    }
    det
}
