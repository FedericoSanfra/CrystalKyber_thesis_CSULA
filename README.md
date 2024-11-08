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


# Overview of the Trellis Code

Il codice Trellis implementa una macchina a stati finiti utilizzata nella decodifica di codici convoluzionali. La struttura `Trellis` è costruita per gestire le transizioni di stato e le metriche di ramo necessarie per calcolare la probabilità logaritmica di ciascuna transizione. Ogni transizione tra stati è definita in una matrice di transizioni, e le funzioni principali permettono di valutare e scegliere il miglior percorso di decodifica basato sulle metriche accumulate.

# Test Cases Summary

| Test Case                               | Description                                                                         | Input Range                       | Output Range                     | Parameters for White-Box Testing                       |
|-----------------------------------------|-------------------------------------------------------------------------------------|------------------------------------|----------------------------------|-------------------------------------------------------|
| `test_transition_to_symbols()`          | Verifica la corretta conversione da transizione a simboli associati (input e output). | State: `[0-3]`, Next state: `[0-3]` | Symbol pair `(i, o)`             | Assicura la correttezza dei simboli associati alle transizioni |
| `test_butterfly()`                      | Testa la funzione `butterfly` per calcolare la metrica massima di ramo da stati precedenti. | Path metrics: `[-inf, inf]`, Branch metrics: `[-inf, inf]` | Max metric                          | Verifica la selezione corretta del percorso massimo |
| `test_possible_transitions()`           | Assicura che `possible_transitions` fornisca tutte le transizioni valide secondo la matrice. | Matrice di transizioni | Array di transizioni valide | Verifica l'elenco corretto delle transizioni valide      |
| `test_future_and_past_states()`         | Testa la corretta associazione tra stati attuali e quelli passati/futuri.           | Stato corrente `[0-3]` | Stati passati/futuri `[0-3]`      | Valida la mappatura tra stati attuali e passati/futuri |


# SISODecoder

## Overview

The `SISODecoder` is a Soft-Input Soft-Output (SISO) decoder that is primarily designed for decoding convolutional codes using a trellis structure. It employs both forward and backward metrics to compute Log-Likelihood Ratios (LLRs) for each symbol, which are used in the decision-making process during decoding.

The decoder operates in a recursive manner, evaluating possible state transitions for each symbol and calculating the corresponding branch metrics. These metrics, combined with the path metrics, are used to produce the final LLRs, which are then used to decode the transmitted message.

### Main Components

- **State Representation**: The decoder uses pairs of states, referred to as `(past_state, future_state)`, to represent the possible transitions between states in the trellis.

- **Branch Metrics**: These metrics are used to evaluate the likelihood of each possible state transition for a given input symbol. They are updated for each state and used to calculate the forward and backward path metrics.

- **Path Metrics**: Both forward and backward path metrics are computed to represent the cumulative metric for each possible path through the trellis.

- **LLR Calculation**: For each symbol, the decoder computes the LLR, which represents the likelihood of each possible bit being transmitted.

The decoder's goal is to minimize the error between the received and expected message by calculating the most likely sequence of symbols.

---

# Test Cases Summary

The following table summarizes the test cases for the `SISODecoder`. These tests ensure the correctness of the decoder's functionality across various scenarios.

| Test Case                               | Description                                                                     | Input Range                     | Output Range                      | Additional Details                                     |
|-----------------------------------------|---------------------------------------------------------------------------------|----------------------------------|------------------------------------|--------------------------------------------------------|
| `test_siso_decoder_creation()`          | Verifies the creation of a new `SISODecoder` instance.                          | Block size: `10`                 | `block_size: 10`, `llr.len: 10`    | Ensures the correct initialization of decoder attributes. |
| `test_init_branch_metrics()`            | Verifies initialization of the branch metrics matrix.                           | Depth: `10`, States: `4`, Transitions: `4` | Matrix with dimensions `[depth][states][transitions]` | Checks the correct dimensions and initialization of branch metrics. |
| `test_init_path_metric()`               | Verifies the initialization of path metrics for states and depth.              | States: `4`, Depth: `10`         | Path metrics matrix with dimensions `[states][depth]` | Ensures path metrics are initialized correctly with proper values. |
| `test_demultiplex()`                    | Verifies the demultiplexing of a vector into a set of 3-tuples.                | Input vector: `[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]` | Vector of tuples: `[(1.0, 2.0, 0.0), (4.0, 5.0, 0.0)]` | Ensures correct splitting and assignment of vector values to tuples. |
| `test_reset()`                          | Verifies that the decoder's metrics are reset to initial values.               | No input                         | Branch, forward, and backward metrics reset to zero or negative infinity | Ensures the decoder correctly resets its state to defaults. |
| `test_expand_states()`                  | Verifies the expansion of state tuples into a vector.                          | State tuple: `(1, 2)`, `(3, 4)`   | Expanded vector: `[1, 2]`, `[3, 4]`  | Ensures correct expansion of state tuples into vectors. |

# Running the Tests

To run the tests in your Rust project, follow these steps:

1. **Ensure you have Rust installed**: If you haven't already, install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Navigate to your project directory**: Open a terminal and change to the directory containing your Rust project.

3. **Run the tests**: Execute the following command in your terminal:

   ```bash
   cargo test