use std::vec::Vec;
use rand::Rng;

pub fn create_random_matrix(size: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| {
        (0..size).map(|_| rng.gen_range(0..10) as f32).collect::<Vec<f32>>()
    }).collect::<Vec<Vec<f32>>>()
}

pub fn print_matrix(matrix: Vec<Vec<f32>>){
    matrix.iter().for_each(|row| {
        print!("| ");
        row.iter().for_each(|&element| {
            print!("{} ", element);
        });
        println!("|")
    });
}
