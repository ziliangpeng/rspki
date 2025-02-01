import random


N_BITS = 1024


def generate_prime(n_bits: int) -> int:
    while True:  # until success
        candidate = random.randint(2 ** (n_bits - 1), 2**n_bits)
        candidate |= 1  # ensure odd

        if miller_rabin(candidate):
            return candidate


def miller_rabin(n: int, k: int = 40) -> bool:
    if n < 2:
        return False
    if n in (2, 3):
        return True
    if n % 2 == 0:
        return False

    # write n - 1 as d * 2^s (d odd)
    s = 0
    d = n - 1
    while d % 2 == 0:
        d //= 2
        s += 1

    # witness loop
    for _ in range(k):
        a = random.randrange(2, n - 1)
        x = pow(a, d, n)  # a^d % n
        if x == 1 or x == n - 1:
            continue
        for _ in range(s - 1):
            x = pow(x, 2, n)  # x ^ 2 % n
            if x == n - 1:
                break
        else:
            return False
    return True


if __name__ == "__main__":
    print(generate_prime(N_BITS))
