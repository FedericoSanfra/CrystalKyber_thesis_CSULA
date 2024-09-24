//! Hashes wrapper
//!
//! Wrapper around the sha3 crates various hash functions

use sha3::{
    digest::{ExtendableOutput, XofReader},
    Digest, Sha3_256, Sha3_512, Shake128, Shake256,
};

/// Shake-128 wrapper
pub fn shake_128(data: &[u8], len: usize) -> Vec<u8> {
    let mut buffer = vec![0; len]; // Allocate buffer of specified length
    let mut shake = Shake128::default(); // Create a new Shake128 instance

    shake.update(data); // Use update instead of input

    // Use `finalize_xof` to obtain the output
    let mut reader = shake.finalize_xof(); // Updated method

    // Read into the buffer to fill it with the customized length from the XOF generator
    reader.read(&mut buffer); //TODO Could Handle potential errors, with expect or something similar

    buffer // Return the filled buffer
}

/// Shake-256 wrapper
pub fn shake_256(data: &[u8], len: usize) -> Vec<u8> {
    let mut buffer = vec![0; len]; // Allocate buffer of specified length
    let mut shake = Shake256::default(); // Create a new Shake256 instance

    shake.update(data); // Use update instead of input

    // Use `finalize_xof` to obtain the output
    let mut reader = shake.finalize_xof(); // Updated method

    // Read into the buffer to fill it with the customized length from the XOF generator
    let bytes_read = reader.read(&mut buffer); // Handle potential errors gracefully
    //TODO handle potential errors or check on the length of buffer created and filled

    buffer // Return the filled buffer
}


/// SHA3-256 wrapper
pub fn sha3_256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::default(); // Create a new SHA3-256 instance
    hasher.update(data); // Use update instead of input
    hasher.finalize().to_vec() // Use finalize to get the hash and convert to Vec<u8>
}

/// SHA3-512 wrapper
pub fn sha3_512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::default(); // Create a new SHA3-512 instance
    hasher.update(data); // Use update instead of input
    hasher.finalize().to_vec() // Use finalize to get the hash and convert to Vec<u8>
}