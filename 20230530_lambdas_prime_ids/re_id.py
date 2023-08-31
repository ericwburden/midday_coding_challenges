def generate_prime_string(n):
    primes = [True for _ in range(n + 1)]
    candidate = 2

    while candidate * candidate <= n:
        if primes[candidate]:
            for idx in range(candidate * candidate, n + 1, candidate):
                primes[idx] = False
        candidate += 1

    return ''.join(str(i) for i in range(2, n + 1) if primes[i])

# A bit of testing shows that this will have enough digits to support
# drawing indexes from 0 to 10_000. Produces a string with 10_009 characters.
# Pre-computing the prime string because doing it on the fly would be silly.
prime_string = generate_prime_string(20231)

def solution(i):
    return prime_string[i:i+5]
    


# Sanity checks to make sure I'm passing the tests, not screwing up the
# ??Python 2.7.13?? syntax, and generating enough prime digits for the
# entire possible input range.
if __name__ == "__main__":
    print solution(0)
    print solution(3)
    print solution(10000)
