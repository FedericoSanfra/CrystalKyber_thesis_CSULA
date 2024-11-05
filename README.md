# ThesisKyber
Thesis project at CSULA on Crystals Kyber implementation, from official documents by round 3 NIST submission. The project aims to study a Turbo code implementation in Rust coding language, setting new challenges for the future cryptographic space applications and communications. by applying the turbo code implementation to an encrypted key with Crystal Kyber we will analyze the impact of a new correlation on Byte Arrays and the performances. 

# Overview of the AWGN Code

The provided Rust code implements an Additive White Gaussian Noise (AWGN) channel simulator. This simulator is designed to convert binary vectors (composed of 0s and 1s) into corresponding symbol representations (-1s and +1s) and apply Gaussian noise to these symbols based on a specified noise level in decibels (dB). The code includes methods for converting binary inputs, creating instances of the AWGN channel with adjustable noise levels, and executing the noise addition process on symbol vectors. To ensure the correctness and robustness of the implementation, a suite of unit tests is included, which checks the behavior of the code under various scenarios, including normal conditions, edge cases, and expected error states.

# Test Cases Summary

| Test Case                               | Description                                                                         | Input Range                     | Output Range                      | Parameters for White-Box Testing                      |
|-----------------------------------------|-------------------------------------------------------------------------------------|----------------------------------|------------------------------------|------------------------------------------------------|
| `test_convert_to_symbols()`             | Tests the conversion of a binary vector to symbols.                                | Binary vector: `[0, 1, 0, 1]`   | Symbol vector: `[-1.0, 1.0, -1.0, 1.0]` | Validates correctness of conversion logic             |
| `test_convert_to_symbols_empty()`       | Tests conversion when provided with an empty vector.                               | Binary vector: `[]`              | Symbol vector: `[]`                | Ensures handling of empty inputs                      |
| `test_execute_with_noise()`             | Tests the application of noise to a vector of symbols, expecting some variation.   | Symbol vector: `[-1.0, 1.0, -1.0, 1.0, 1.0]` | Varies based on noise application  | Verifies noise introduction logic                      |
| `test_execute_high_noise()`             | Tests behavior under high noise conditions (0 dB), expecting significant changes.  | Symbol vector: `[-1.0, 1.0, -1.0, 1.0, 1.0]` | Significant variations from originals | Ensures expected behavior under high noise levels    |
| `test_execute_low_noise()`              | Tests behavior under low noise conditions (40 dB), expecting minimal changes.      | Symbol vector: `[-1.0, 1.0, -1.0, 1.0, 1.0]` | Minimal variations from originals    | Verifies minimal noise effect                          |
| `test_execute_empty_vector()`           | Tests execution on an empty vector, expecting an empty output.                    | Symbol vector: `[]`              | Noisy output: `[]`                 | Ensures robustness against empty input vectors        |

# Overview of the RSC Code

The provided Rust code implements a Recursive Systematic Convolutional (RSC) encoder. This encoder processes input binary vectors, computes the corresponding parity bits, and maintains its state using internal registers. It includes methods for resetting the state of the registers, pushing input bits to generate parity, terminating the encoding process to flush the registers, and executing the full encoding operation on a given input vector. A suite of unit tests is included to ensure the correctness and robustness of the implementation, covering various scenarios including normal operations, edge cases, and expected error states.

# Test Cases Summary

| Test Case                               | Description                                                                         | Input Range                     | Output Range                      | Parameters for White-Box Testing                      |
|-----------------------------------------|-------------------------------------------------------------------------------------|----------------------------------|------------------------------------|------------------------------------------------------|
| `test_reset()`                         | Verifies that the reset method correctly resets the registers to initial values.   | No input                        | Registers: `[0, 0]`               | Validates register state before and after reset       |
| `test_push()`                          | Tests the `push` method for a single input bit, ensuring the correct parity bit is returned. | Input bit: `1` or `0`          | Parity bit (0 or 1)               | Checks register values after processing a bit         |
| `test_terminate()`                     | Verifies that the `terminate` method returns the correct value from the last register. | No input                        | Last register value (u8)          | Validates register state before and after termination  |
| `test_execute()`                       | Tests the complete encoding process on an input vector, ensuring correct output lengths. | Vector of bits: `[1, 1, 0, 1]` | Encoded bits length: `6`          | Checks lengths of encoded and systematic outputs       |

# Running the Tests

To run the tests in your Rust project, follow these steps:

1. **Ensure you have Rust installed**: If you haven't already, install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Navigate to your project directory**: Open a terminal and change to the directory containing your Rust project.

3. **Run the tests**: Execute the following command in your terminal:

   ```bash
   cargo test