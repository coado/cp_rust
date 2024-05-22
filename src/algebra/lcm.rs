use crate::algebra::gcd::gcd;

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
