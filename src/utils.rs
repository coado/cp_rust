use std::fmt::Display;

pub fn print_2d_vector<T: Display>(vector: &Vec<Vec<T>>) {
    for row in vector {
        for val in row {
            print!("{}, ", val);
        }
        println!();
    }
}

pub fn print_vector<T: Display>(vector: &Vec<T>) {
    for val in vector {
        print!("{}, ", val);
    }
}
