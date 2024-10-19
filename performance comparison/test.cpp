#include <iostream>
#include <vector>
#include <chrono>
#include <cmath>

// Criba de Eratóstenes optimizada
std::vector<int> sieve_of_eratosthenes(int limit) {
    std::vector<bool> is_prime(limit + 1, true);
    is_prime[0] = is_prime[1] = false;

    std::vector<int> primes;
    primes.reserve(limit / 10);  // Prealocar espacio basado en una estimación

    std::cout << "Starting sieve..." << std::endl;

    for (int num = 2; num * num <= limit; ++num) {
        if (is_prime[num]) {
            for (int multiple = num * num; multiple <= limit; multiple += num) {
                is_prime[multiple] = false;
            }
        }
    }

    // Recoger todos los números primos
    for (int num = 2; num <= limit; ++num) {
        if (is_prime[num]) {
            primes.push_back(num);
        }
    }

    std::cout << "Sieve finished..." << std::endl;
    return primes;
}

int main() {
    std::cout << "Program started..." << std::endl;

    auto start = std::chrono::high_resolution_clock::now();

    std::vector<int> primes = sieve_of_eratosthenes(10000000);

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> duration = end - start;

    std::cout << "Found " << primes.size() << " prime numbers." << std::endl;
    std::cout << "Execution time: " << duration.count() << " seconds." << std::endl;

    return 0;
}
