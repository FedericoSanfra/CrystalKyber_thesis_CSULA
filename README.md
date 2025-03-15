# CRYSTALS-Kyber & Turbo Codes in Rust

## Overview

This project implements **CRYSTALS-Kyber**, a **post-quantum cryptographic algorithm**, along with **Turbo Codes**, an advanced **error correction technique** used in communication systems.

- **Kyber** is a lattice-based key encapsulation mechanism (KEM) that provides secure encryption resistant to quantum attacks.
- **Turbo Codes** use **Recursive Systematic Convolutional (RSC) encoders** and iterative decoding to achieve near-optimal error correction performance in noisy channels.

The goal of this project is to implement a simulation of a Kyber post-quantum encryption scheme, to be used in satellite communication in space. Rust is the new programming language frontier and is now considered the future of space missions.
The use of Turbo codes allows us to detect and correct errors cause by radiations in space, with great performances. 
## Project Structure

The repository is structured into two main components:

### `src/kcimpl/`
This folder contains the **Kyber cryptographic implementation**.

- **`kem/`**
    - `mod.rs`: Main module for the **Kyber Key Encapsulation Mechanism (KEM)**.
- 
- **`functions/`**
    - `mod.rs`: module handling.
    - `compress.rs`: Implements Kyber's compress function, defined in the official documentation.
    - `encode.rs`: Implements Kyber's encode function, defined in the official documentation.
    - `hash.rs`: Employes hash functions from SHA-3 standard.
    - `ntt.rs`: Implements NTT Number Theoretic Transform.
    - `utils.rs`: Implements other side functions, such as hash and PRF.

- **`pke/`**
    - `mod.rs`: Implements **Public Key Encryption (PKE)** using Kyber.

- **`structures/`**
    - `algebraics/`: Contains mathematical structures used in Kyber.
        - `bytearray.rs`: Handles byte-level operations.
        - `primefield.rs`: Implements operations over finite fields, crucial for cryptographic computations.

---

### `src/turbof/`
This folder contains the **Turbo Codes implementation** for error correction.

- **`bsc_channel.rs`**
    - Simulates a **Binary Symmetric Channel (BSC)**, introducing controlled errors for testing.

- **`mapints.rs`**
    - Implements **interleaving** functions, which permute the input bits to improve error correction.

- **`rsc_encoder.rs`**
    - Implements the **Recursive Systematic Convolutional (RSC) Encoder**, the core building block of Turbo Codes.

- **`siso_decoder.rs`**
    - Implements the **Soft-Input Soft-Output (SISO) Decoder**, which performs iterative decoding.

- **`turbo_encoder.rs`**
    - Implements the **Turbo Encoder**, which consists of two RSC encoders and an interleaver.

- **`turbo_decoder.rs`**
    - Implements the **Turbo Decoder**, performing **iterative decoding** for error correction.

- **`turbo_simulation.rs`**
    - Runs simulations to evaluate the performance of the Turbo Codes in different channel conditions.

- **`utils.rs`**
    - Provides helper functions for Turbo Codes processing.

---

### Other Files:

- **Test Files (`tests/`)**
    - `integration_test.rs`: Contains integration tests for the Turbo Codes and Kyber implementations.

- **Data Files (`.txt`)**
    - `input.txt`: Example input data.
    - `interleaver.txt`: Defines **interleaving patterns** for Turbo Codes.
    - `kyber_keys.txt`: Stores cryptographic keys used in the Kyber algorithm.
    - `output.txt`: Stores probabilities after correction and number of errors, both bit and block error rate and the real uncoded probability.
- **Kyber Test (`.rs`)**
    - `lib.rs`: Stores tests for Kyber KEM and PKE schemes. It can be used to simulate the whole communication between 
  Alice and Bob.

---

## How to Use
- In `lib.rs` the user can find the setting for CRYSTALS-Kyber simulation, choosing the complexity of the algorithm and 
running tests for KEM and PKE.
- In integration_tests, there are different tests in order to test the Turbo code algorithm. `test_turbo_simulation_dyn()` is the ultimate test, that is made of a for cycle
in order to run the simulation many times and gather as much data as possible. Parameters that can be modified:
    - simulation_length
    - block_size
    - error_probability
The interleaver is set with the same length of the turbo-block. The interleaver is coming from the input file interleaver.txt 
of 64000 bits length. In the output file is stored the result of simulations. By assuming that the random generated input is 
made of uncorrelated bits, there is no difference between a random generated input sequence and a kyber's key (public key, secret key or cyphertext).
Anyway, in the `turbo_simulation.rs` file the code for using kyber keys as input, is commented and can be used, in lines: 81-82, by using an input.txt file.

      

