use std::time::Instant;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = Vec::with_capacity(limit / 10);  // Prealocar memoria para los primos

    for num in 2..=limit {
        if is_prime[num] {
            primes.push(num);
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    primes
}

fn main() {
    let start = Instant::now();

    let primes = sieve_of_eratosthenes(10000000);  // Calcula los primos hasta 1,000,000

    let duration = start.elapsed();
    println!("Found {} prime numbers.", primes.len());
    println!("Execution time: {:.2?} seconds.", duration);
}
