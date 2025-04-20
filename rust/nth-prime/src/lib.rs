pub fn nth(n: u32) -> u32 {
    let mut primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    if n as usize >= primes.len() {
        let mut candidate = primes.last().unwrap() + 2;
        while n as usize >= primes.len() {
            if is_prime(candidate, &primes) {
                primes.push(candidate);
            }
            candidate += 2;
        }
    }

    primes[n as usize]
}

fn is_prime(n: u32, primes: &[u32]) -> bool {
    let sqrt_n = (n as f64).sqrt() as u32;
    primes
        .iter()
        .take_while(|&&p| p <= sqrt_n)
        .all(|&p| n % p != 0)
}
