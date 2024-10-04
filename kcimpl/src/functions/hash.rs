//! Hashes wrapper
//!
//! Wrapper around the sha3 crates various hash functions
use sha3::digest::{Update, ExtendableOutput, XofReader};
use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
use std::io::Read;
/// Shake-128 wrapper
pub fn shake_128(data: &[u8], len: usize) -> Vec<u8> {
    let mut buffer = vec![0; len]; // Allocate buffer of specified length
    let mut shake = Shake128::default(); // Create a new Shake128 instance

    // Disambiguate the `update` method by specifying the trait
    Update::update(&mut shake, data);

    // Use `finalize_xof` to obtain the output
    let mut reader = shake.finalize_xof(); // Updated method

    // Read into the buffer to fill it with the customized length from the XOF generator
    reader.read_exact(&mut buffer).expect("Failed to read from SHAKE128 XOF");

    buffer // Return the filled buffer
}

/// Shake-256 wrapper
pub fn shake_256(data: &[u8], len: usize) -> Vec<u8> {
    let mut buffer = vec![0; len]; // Allocate buffer of specified length
    let mut shake = Shake256::default(); // Create a new Shake256 instance

    // Disambiguate the `update` method by specifying the trait
    Update::update(&mut shake, data);

    // Use `finalize_xof` to obtain the output
    let mut reader = shake.finalize_xof(); // Updated method

    // Read into the buffer to fill it with the customized length from the XOF generator
    reader.read_exact(&mut buffer).expect("Failed to read from SHAKE256 XOF");

    buffer // Return the filled buffer
}

/// SHA3-256 wrapper
pub fn sha3_256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::default(); // Create a new SHA3-256 instance

    // Disambiguate the `update` method by specifying the trait
    Digest::update(&mut hasher, data);

    hasher.finalize().to_vec() // Use finalize to get the hash and convert to Vec<u8>
}

/// SHA3-512 wrapper
pub fn sha3_512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::default(); // Create a new SHA3-512 instance

    // Disambiguate the `update` method by specifying the trait
    Digest::update(&mut hasher, data);

    hasher.finalize().to_vec() // Use finalize to get the hash and convert to Vec<u8>
}
