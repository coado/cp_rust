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

pub(crate) mod bitmasks {
    pub fn lsone(n: i32) -> i32 {
        return n & -n;
    }

    pub fn enumerate_subsets(mask: i32) -> Vec<i32> {
        let mut subset = mask;
        let mut res = Vec::new();
        while subset > 0 {
            res.push(subset);
            subset = (subset - 1) & mask;
        }

        return res;
    }

    // count how many bits are on in n
    pub fn popcount(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            count += 1;
            n &= n - 1;
        }

        return count;
    }

    pub fn is_on(mask: i32, i: i32) -> bool {
        return mask & (1 << i) > 0;
    }

    // count trailing zeros
    pub fn ctz(mut n: i32) -> i32 {
        let mut count = 0;
        while n & 1 == 0 {
            count += 1;
            n >>= 1;
        }

        return count;
    }

    pub fn turn_on(mask: i32, i: i32) -> i32 {
        return mask | (1 << i);
    }

    pub fn turn_off(mask: i32, i: i32) -> i32 {
        return mask & !(1 << i);
    }

    pub fn toggle(mask: i32, i: i32) -> i32 {
        return mask ^ (1 << i);
    }

    pub fn modulo(mask: i32, k: i32) -> i32 {
        return mask & ((1 << k) - 1);
    }

    pub fn is_power_of_two(mask: i32) -> bool {
        return mask & (mask - 1) == 0;
    }
}
