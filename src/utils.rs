use std::fmt::Display;

pub fn print_2d_vector<T: Display>(vector: &Vec<Vec<T>>) {
    for i in 0..vector.len() {
        for j in 0..vector[i].len() {
            print!("{}, ", vector[i][j]);
        }
        println!();
    }
}

pub fn print_vector<T: Display>(vector: &Vec<T>) {
    for i in 0..vector.len() {
        print!("{}, ", vector[i]);
    }
}
