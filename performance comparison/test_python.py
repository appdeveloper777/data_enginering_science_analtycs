import time

def sieve_of_eratosthenes(limit):
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    primes = []

    for num in range(2, limit + 1):
        if sieve[num]:
            primes.append(num)
            for multiple in range(num * num, limit + 1, num):
                sieve[multiple] = False

    return primes

if __name__ == "__main__":
    start = time.time()
    
    primes = sieve_of_eratosthenes(10000000)
    
    end = time.time()
    print(f"Found {len(primes)} prime numbers.")
    print(f"Execution time: {end - start} seconds.")
