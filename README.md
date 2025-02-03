# rspki
rust implementation of pki (public key infrastructure); educational purpose

## Goal
- minimal dependency. implement all logic from first principle
- clean and minimum code, focus on core algorithm

## Plans
- x.509 cert generation
- public / private key pair generation (RSA/ECC?)
  - generate pub key from pri key
- cert signing request (CSR) handling
- cert authority functionality
- basic cert validation logic

---

---


### Feb 2, 2025
Started this project to implement pki using rust. Have no rust or pki experience before, but Cursor, Perplexity, DeepSeek-R1, OpenAI o3-mini (just released this weekend) is extremely helpful.

Did a little proof of concept in python, then implemented rust code to generate large primes. This is completed in a weekend and it feels good. Now I can say I know rust, and I know pki.

Code is still rather slow and unoptimized. Did a little `cargo flamegraph` and locked down the hot spot in the rem (%) method.
