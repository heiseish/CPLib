fn sieve<const N: usize>() -> Vec<usize> {
    let mut primes = vec![];
    let mut is_prime = vec![true; N];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..N {
        if is_prime[i] {
            let li = i as i64;
            let mut j = li * li;
            while j < N as i64 {
                is_prime[j as usize] = false;
                j += i as i64;
            }
            primes.push(i);
        }
    }
    primes
}
