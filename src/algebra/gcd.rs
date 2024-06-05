/// Euclid's algorithm for finding the greatest common divisor of two numbers
/// Time complexity is roughly O(log(min(a, b)))
/// <https://en.wikipedia.org/wiki/Euclidean_algorithm>
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn ext_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }

    let mut a = a;
    let mut b = b;
    let mut xx = 0;
    let mut yy = 1;
    let mut x = 1;
    let mut y = 0;

    while b != 0 {
        let q = a / b;
        let mut t = b;
        b = a % b;
        a = t;
        t = xx;
        xx = x - q * xx;
        x = t;
        t = yy;
        yy = y - q * yy;
        y = t;
    }

    (a, x, y)
}

#[cfg(test)]
mod test {
    use super::{ext_euclid, gcd};

    #[test]
    fn test_gcd() {
        let result = gcd(8, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_gcd_negative() {
        let result = gcd(-8, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_gcd_zero() {
        let result = gcd(0, 12);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_gcd_zero_both() {
        let result = gcd(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_gcd_negative_both() {
        let result = gcd(-8, -12);
        assert_eq!(result, -4);
    }

    #[test]
    fn test_gcd_large() {
        let result = gcd(1_000_000_000, 1_000_000_000);
        assert_eq!(result, 1_000_000_000);
    }

    #[test]
    fn test_ext_euclid() {
        let result = ext_euclid(25, 18);
        assert_eq!(result, (1, -5, 7));
    }

    #[test]
    fn test_ext_euclid_negative() {
        let result = ext_euclid(-25, 18);
        assert_eq!(result, (1, 5, 7));
    }

    #[test]
    fn test_ext_euclid_zero() {
        let result = ext_euclid(0, 18);
        assert_eq!(result, (18, 0, 1));
    }

    #[test]
    fn test_ext_euclid_zero_both() {
        let result = ext_euclid(0, 0);
        assert_eq!(result, (0, 1, 0));
    }

    #[test]
    fn test_ext_euclid_negative_both() {
        let result = ext_euclid(-25, -18);
        assert_eq!(result, (-1, -5, 7));
    }
}
