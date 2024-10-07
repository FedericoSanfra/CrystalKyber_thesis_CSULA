# ThesisKyber
Thesis project at Cal State University of Los Angeles on the Crystals Kyber algorithm.
I started from an online Rust implementation of the algorithm, based on official supporting documentation from the round 2 NIST submission.
The aim of the project is to develop a new implementation in Rust language, to improve safety levels and robustness against radiation in space applications. The algorithm implementation is based on the new modifications found in the round 3 submission to NIST and exploits well-known techniques of soft encoding and ECC to recover from errors and bit-flipping caused by failure events.

# Test Suite for PKE Sharding Reed-Solomon

## Description

This section contains a detailed table of the unit tests we have written to verify the correct implementation of the key sharding and reconstruction functionalities using the Reed-Solomon encoder. The unit tests are written in **white-box testing** mode, including edge cases and various useful scenarios.

## Test Table

| #  | Test Name                                      | Tested Range                          | Description                                                                                                                                               |
|----|------------------------------------------------|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1  | `test_shard_key_valid_input`                   | 64-byte key, 5 data shards, 3 parity shards | Tests the normal case of sharding with a 64-byte key and verifies that the number of shards and their sizes are correct.                                 |
| 2  | `test_shard_key_edge_case_empty_key`           | Empty key, 5 data shards, 3 parity shards | Checks the behavior with an empty key, ensuring that 8 shards are generated, all of which are empty.                                                       |
| 3  | `test_shard_key_edge_case_small_key`           | 3-byte key, 5 data shards, 3 parity shards | Tests the behavior with a key smaller than the size of a single shard. Verifies that the first shard contains the key and the others are empty.          |
| 4  | `test_reconstruct_key_valid_input`             | 64-byte key, 5 data shards, 3 parity shards | Verifies the correct reconstruction of the key using all the generated shards without any being missing.                                                  |
| 5  | `test_reconstruct_key_with_missing_shards`     | 64-byte key, 2 missing shards        | Checks that the key can be reconstructed correctly even if up to 2 shards are missing, utilizing the 3 parity shards to recover the data.                 |
| 6  | `test_reconstruct_key_with_too_many_missing_shards` | 64-byte key, 3 missing shards        | Tests the error case when more shards are missing than can be recovered (3 out of 8). Verifies that the system fails as expected.                         |
| 7  | `test_shard_key_with_various_shard_sizes`      | 128-byte key, various combinations of data and parity shards | Tests the functionality with various shard combinations (2 data + 2 parity, 4 data + 2 parity, 5 data + 3 parity, 8 data + 4 parity), ensuring that the number of generated shards is correct. |

## Detailed Description of Tests

### 1. `test_shard_key_valid_input`

- **Input**: 64-byte key, 5 data shards, 3 parity shards.
- **Objective**: Verify that the key is correctly divided into the appropriate data and parity shards. It checks that the number of generated shards is correct and that each shard contains the expected data.

### 2. `test_shard_key_edge_case_empty_key`

- **Input**: Empty key, 5 data shards, 3 parity shards.
- **Objective**: Test the behavior when the key is empty. Even with an empty key, it checks that 8 shards are generated and that all are empty.

### 3. `test_shard_key_edge_case_small_key`

- **Input**: 3-byte key, 5 data shards, 3 parity shards.
- **Objective**: Test the behavior with a key smaller than the size of the individual shards. It verifies that the first shards contain the key data and that the remaining shards are filled as expected.

### 4. `test_reconstruct_key_valid_input`

- **Input**: 64-byte key, 5 data shards, 3 parity shards.
- **Objective**: Verify that the original key can be correctly reconstructed using all generated shards (without losses).

### 5. `test_reconstruct_key_with_missing_shards`

- **Input**: 64-byte key, with 2 missing shards.
- **Objective**: Verify that the key can be reconstructed even if up to 2 shards are missing (the maximum number of missing shards recoverable with the 3 parity shards).

### 6. `test_reconstruct_key_with_too_many_missing_shards`

- **Input**: 64-byte key, with 3 missing shards.
- **Objective**: Verify that if too many shards are missing (more than can be recovered from the parity shards), the system raises an exception and fails as expected.

### 7. `test_shard_key_with_various_shard_sizes`

- **Input**: 128-byte key, with various combinations of data and parity shards (2 data + 2 parity, 4 data + 2 parity, 5 data + 3 parity, 8 data + 4 parity).
- **Objective**: Verify that the `shard_key` function works correctly with different shard configurations, ensuring that the number of generated shards is always correct.

## Running the Tests

To run all the tests, make sure you have Rust installed and compile the project with the following commands:

```bash
cargo build
cargo test
