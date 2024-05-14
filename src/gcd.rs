// Euclid's algorithm
// https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
