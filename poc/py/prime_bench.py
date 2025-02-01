import time
from prime import generate_prime

bits = [int(128 * pow(1.05, i)) for i in range(64)]

for b in bits:
    sec = []
    for _ in range(10):
        start = time.time()
        generate_prime(b)
        end = time.time()
        sec.append(end - start)
    print(f"{b} bits; avg time {sum(sec) / len(sec):.2f}s; max {max(sec):.2f}s; min {min(sec):.2f}s")
