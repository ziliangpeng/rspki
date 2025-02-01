import prime


def egcd(a, b):
    # Extended Euclidean Algorithm: returns (g, x, y) such that ax + by = g = gcd(a, b)
    if a == 0:
        return (b, 0, 1)
    g, x1, y1 = egcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return (g, x, y)


def modinv(a, m):
    # Compute the modular inverse of a modulo m, if it exists
    g, x, _ = egcd(a, m)
    if g != 1:
        raise Exception("Modular inverse does not exist")
    return x % m


def generate_key(n_bits: int) -> tuple[int, int]:
    p = prime.generate_prime(n_bits)
    q = prime.generate_prime(n_bits)
    n = p * q
    phi = (p - 1) * (q - 1)
    expo = 65537

    # Optionally, check gcd(e, phi) == 1; if not, choose a different e
    if egcd(expo, phi)[0] != 1:
        raise Exception("e and phi(n) are not coprime!")

    d = modinv(expo, phi)

    return (n, expo), (n, d)


if __name__ == "__main__":
    pub, priv = generate_key(1024)
    print("public key:", pub)
    print("private key:", priv)
    # TODO: convert to PEM format and write to file
