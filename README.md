# ThesisKyber
Thesis project at Cal State University of Los Angeles on the Crystals Kyber algorithm.

I started from an online Rust implementation of the algorithm, based on official supporting documentation from the round 2 NIST submission. The aim of the project is to develop a new implementation in the Rust language, improving safety levels and robustness against radiation in space applications. The algorithm implementation builds on modifications introduced in the round 3 submission to NIST, incorporating techniques like soft encoding and ECC (Error Correction Code) to recover from bit-flipping caused by failure events.

# Test Suite for PKE Sharding Reed-Solomon

## Description

This section outlines the unit tests verifying the correct implementation of key sharding and reconstruction functionalities using the Reed-Solomon encoder. These tests are designed using **white-box testing**, covering edge cases and other relevant scenarios to ensure robustness.

## Test Table

| #  | Test Name                                      | Tested Range                                     | Description                                                                                                                                              |
|----|------------------------------------------------|--------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1  | `test_valid_encoding`                          | Key: 10 bytes, 5 data shards, 3 parity shards    | Verifies that the encoding function correctly shards the key and produces the expected number of shards with the correct sizes.                           |
| 2  | `test_empty_byte_array`                        | Empty key, 3 data shards, 2 parity shards        | Ensures the encoder returns an error for an empty key, validating the `EmptyDataError` behavior.                                                          |
| 3  | `test_invalid_shard_sizes`                     | Key: 5 bytes, 2 data shards, 3 parity shards     | Tests invalid shard size configuration where the number of parity shards exceeds data shards, expecting an `InvalidShardsSize` error.                     |
| 4  | `test_zero_data_shards`                        | Key: 5 bytes, 0 data shards, 3 parity shards     | Verifies that passing zero data shards leads to an `InvalidShardsSize` error, as data shards must be greater than zero.                                   |
| 5  | `test_zero_parity_shards`                      | Key: 5 bytes, 3 data shards, 0 parity shards     | Tests the case of zero parity shards, which should return an `InvalidShardsSize` error, as parity shards must also be greater than zero.                  |
| 6  | `test_encoding_with_exact_shards`              | Key: 6 bytes, 3 data shards, 2 parity shards     | Verifies that the encoding works when the key size is divisible by the number of data shards, ensuring even shard sizes.                                  |
| 7  | `test_encoding_with_non_divisible_shards`      | Key: 7 bytes, 3 data shards, 2 parity shards     | Checks that encoding handles cases where the key size is not divisible by the data shards, ensuring shards are padded appropriately.                      |

## Detailed Description of Tests

### 1. `test_valid_encoding`

- **Input**: Key (10 bytes), 5 data shards, 3 parity shards.
- **Objective**: Verifies that the key is correctly encoded into data and parity shards. It ensures that the number of shards matches the expected count and that their sizes are calculated as expected.

### 2. `test_empty_byte_array`

- **Input**: Empty key, 3 data shards, 2 parity shards.
- **Objective**: Ensures that encoding fails with an `EmptyDataError` when the key is empty, and no shards are generated.

### 3. `test_invalid_shard_sizes`

- **Input**: Key (5 bytes), 2 data shards, 3 parity shards.
- **Objective**: Verifies that an invalid configuration, where the number of parity shards exceeds the data shards, triggers the `InvalidShardsSize` error.

### 4. `test_zero_data_shards`

- **Input**: Key (5 bytes), 0 data shards, 3 parity shards.
- **Objective**: Ensures that encoding fails with an `InvalidShardsSize` error when no data shards are provided.

### 5. `test_zero_parity_shards`

- **Input**: Key (5 bytes), 3 data shards, 0 parity shards.
- **Objective**: Verifies that encoding fails with an `InvalidShardsSize` error when no parity shards are provided.

### 6. `test_encoding_with_exact_shards`

- **Input**: Key (6 bytes), 3 data shards, 2 parity shards.
- **Objective**: Verifies that encoding works correctly when the key size is divisible evenly among the data shards.

### 7. `test_encoding_with_non_divisible_shards`

- **Input**: Key (7 bytes), 3 data shards, 2 parity shards.
- **Objective**: Ensures that encoding works when the key size is not evenly divisible among the data shards, verifying that the shards are padded as necessary.

## Running the Tests

To execute all tests, ensure Rust is installed and run the following commands:

```bash
cargo build
cargo test
