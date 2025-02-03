from sympy import isprime
n=input()
if n.startswith('0x'): n=int(n, 16)
else: n=int(n)
print(n, 'is prime' if isprime(n) else 'is not prime')
