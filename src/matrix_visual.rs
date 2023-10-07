use std::vec::Vec;
use rand::Rng;
pub fn create_random_matrix(dest: &mut Vec<Vec<f32>>, size: usize){
    let mut rng = rand::thread_rng();
    *dest = vec![vec![0.0; size]; size];
    //dest.resize(size, vec![0f32, size as f32]);
    for i in 0..size{
        for j in 0..size {dest[i][j] = rng.gen_range(1..10) as f32}
    }
}

pub fn print_matrix(matrix: Vec<Vec<f32>>, size: usize){
    for i in 0..size{
        print!("| ");
        for j in 0..size {
            print!("{} ", matrix[i][j]);
        }
    print!("|\n");
    }
}
