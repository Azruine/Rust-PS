use std::{io::Write, vec};

/// # Sieve of Eratosthenes
///
/// ## Arguments
///
/// None
///
/// ## Returns
///
/// A vector of prime numbers
///
/// ## Examples
///
/// ```
/// let primes = sieve();
/// ```
fn sieve() -> Vec<i32> {
    const MAX: usize = 1_000_100;

    let mut lowest_prime = vec![0; MAX];
    let mut primes = Vec::new();

    for i in 2..MAX {
        if lowest_prime[i] == 0 {
            lowest_prime[i] = i as i32;
            primes.push(i as i32);
        }
        for &p in &primes {
            let x = i * p as usize;
            if x >= MAX || p > lowest_prime[i] {
                break;
            }
            lowest_prime[x] = p;
        }
    }

    primes
}

fn main() {
    let primes = sieve();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, m): (i32, i32) = (iter.next().unwrap(), iter.next().unwrap());

    let lower_bound = primes.iter().position(|&x| x >= n).unwrap();
    let upper_bound = primes.iter().position(|&x| x > m).unwrap();

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    for prime in primes[lower_bound..upper_bound].iter() {
        writeln!(out, "{}", prime).unwrap();
    }
}
