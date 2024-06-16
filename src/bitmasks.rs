#[allow(dead_code)]
pub(crate) fn lsone(n: i32) -> i32 {
    n & -n
}

#[allow(dead_code)]
pub(crate) fn enumerate_subsets(mask: i32) -> Vec<i32> {
    let mut subset = mask;
    let mut res = Vec::new();
    while subset > 0 {
        res.push(subset);
        subset = (subset - 1) & mask;
    }

    res
}

#[allow(dead_code)]
// count how many bits are on in n
pub(crate) fn popcount(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += 1;
        n &= n - 1;
    }

    count
}

#[allow(dead_code)]
pub(crate) fn is_on(mask: i32, i: i32) -> bool {
    mask & (1 << i) > 0
}

#[allow(dead_code)]
// count trailing zeros
pub(crate) fn ctz(mut n: i32) -> i32 {
    let mut count = 0;
    while n & 1 == 0 {
        count += 1;
        n >>= 1;
    }

    count
}

#[allow(dead_code)]
pub(crate) fn turn_on(mask: i32, i: i32) -> i32 {
    mask | (1 << i)
}

#[allow(dead_code)]
pub(crate) fn turn_off(mask: i32, i: i32) -> i32 {
    mask & !(1 << i)
}

#[allow(dead_code)]
pub(crate) fn toggle(mask: i32, i: i32) -> i32 {
    mask ^ (1 << i)
}

#[allow(dead_code)]
pub(crate) fn modulo(mask: i32, k: i32) -> i32 {
    mask & ((1 << k) - 1)
}

#[allow(dead_code)]
pub(crate) fn is_power_of_two(mask: i32) -> bool {
    mask & (mask - 1) == 0
}
