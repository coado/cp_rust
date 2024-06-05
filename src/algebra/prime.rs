/// Sieve of Eratosthenes is an algorithm that finds all prime numbers up to a given limit.
/// Time complexity is roughly O(Nlog(log(N)))
pub fn sieve(n: usize) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];

    if n < 2 {
        return res;
    }

    let mut is_prime = vec![true; n + 1];
    res.push(2);

    for i in (3..=n).step_by(2) {
        if is_prime[i] {
            res.push(i as u64);
            // If the number is prime, the next multiple that is not prime is its square
            for j in (i * i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    res
}

/// Checks if a given number is prime using trial division
/// Time complexity is roughly O(sqrt(N))
pub fn is_prime(n: u64) -> bool {
    if n < 3 {
        return n == 2;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

/// Returns list of prime factors of a given number
/// Uses the sieve of Eratosthenes to find all prime numbers up to the square root of the given number
/// Time complexity is roughly O(Nlog(log(N)) + sqrt(N)/ln(sqrt(N))
pub fn prime_factors(n: u64) -> Vec<u64> {
    let p = sieve(n as usize);

    let mut res = vec![];
    let mut n = n;
    let mut i = 0;

    while i < p.len() && p[i] * p[i] <= n {
        while n % p[i] == 0 {
            n /= p[i];
            res.push(p[i]);
        }
        i += 1;
    }

    if n != 1 {
        res.push(n);
    }

    res
}

/// Returns the number of positive integers < N that are coprime to N
pub fn euler_phi(n: u64) -> u64 {
    let p = sieve(n as usize);

    let mut n = n;
    let mut ans = n;
    let mut i = 0;

    while i < p.len() && p[i] * p[i] <= n {
        if n % p[i] == 0 {
            ans -= ans / p[i];
        }
        while n % p[i] == 0 {
            n /= p[i];
        }

        i += 1;
    }

    if n != 1 {
        ans -= ans / n;
    }

    ans
}

#[cfg(test)]
mod test {
    use super::{euler_phi, prime_factors, sieve};
    use std::time::Instant;

    #[test]
    fn test_euler_phi() {
        let res = euler_phi(10);
        assert_eq!(res, 4);

        let res = euler_phi(12);
        assert_eq!(res, 4);

        let res = euler_phi(13);
        assert_eq!(res, 12);

        let res = euler_phi(14);
        assert_eq!(res, 6);

        let res = euler_phi(15);
        assert_eq!(res, 8);

        let res = euler_phi(16);
        assert_eq!(res, 8);

        let res = euler_phi(17);
        assert_eq!(res, 16);

        let res = euler_phi(18);
        assert_eq!(res, 6);

        let res = euler_phi(19);
        assert_eq!(res, 18);

        let res = euler_phi(20);
        assert_eq!(res, 8);

        let res = euler_phi(36);
        assert_eq!(res, 12);
    }

    #[test]
    fn test_prime_factors() {
        let res = prime_factors(210);
        assert_eq!(res, vec![2, 3, 5, 7]);

        let res = prime_factors(315);
        assert_eq!(res, vec![3, 3, 5, 7]);

        let res = prime_factors(100);
        assert_eq!(res, vec![2, 2, 5, 5]);

        let res = prime_factors(1);
        assert_eq!(res, vec![]);

        let res = prime_factors(2);
        assert_eq!(res, vec![2]);

        let res = prime_factors(3);
        assert_eq!(res, vec![3]);

        let res = prime_factors(4);
        assert_eq!(res, vec![2, 2]);
    }

    #[test]
    fn test_sieve_number_of_primes() {
        let start = Instant::now();
        let result = sieve(i32::pow(10, 7) as usize);
        let duration = start.elapsed();
        println!("duration: {:?}", duration);
        assert_eq!(result.len(), 664579);
    }

    #[test]
    fn test_sieve_lower_than_two() {
        let result = sieve(1);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_sieve_for_two() {
        let result = sieve(2);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_sieve_for_three() {
        let result = sieve(3);
        assert_eq!(result.len(), 2);
    }
}
