use std::time::Instant;

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn calculate_primes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::with_capacity(78498);  // Prealocar el espacio esperado
    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

fn main() {
    let start = Instant::now();

    let primes = calculate_primes(1000000);

    let duration = start.elapsed();
    println!("Found {} prime numbers.", primes.len());
    println!("Execution time: {:.2?} seconds.", duration);
}
