pub fn sieve(n: usize) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];

    if n < 2 {
        return res;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    res.push(2);

    // Mark all even numbers as non-prime except 2
    for i in (4..n).step_by(2) {
        is_prime[i] = false;
    }

    let mut i = 3;

    // If the number is prime, the next multiple that is not prime is its square
    while i * i <= n {
        if is_prime[i] {
            res.push(i as u64);
            // Except 2 all other primes are odd
            // So we can skip even multiples
            for j in (i * i..n).step_by(2 * i) {
                is_prime[j] = false;
            }
        }

        i += 2;
    }

    res
}

#[cfg(test)]
mod test {
    use super::sieve;

    #[test]
    fn test_sieve() {
        let result = sieve(10);
        assert_eq!(result, vec![2, 3, 5, 7]);
    }
}
