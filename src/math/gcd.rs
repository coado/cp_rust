// Euclid's algorithm
// https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod test {
    use super::gcd;

    #[test]
    fn test_gcd() {
        let result = gcd(8, 12);
        assert_eq!(result, 4);
    }
}
