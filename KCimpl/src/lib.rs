//! This is documentation for the `KCimpl` crate.
//!
//! # Introduction
//! `KCimpl` is an implementation of Crystals-Kyber , a post-quantum
//! candidate submitted to NIST for standardization.
//!
//! This crate provides public-key encryption (`PKE`) and key encapsulation (`KEM`).
//!
//! # Examples
//!
//! For the KEM:
//!
//! ```rust
//! use kcimpl::kyber512kem;
//! let kem = kyber512kem();
//!
//! // Alice runs keygen, publishes pk. Value sk is secret
//! let (sk, pk) = kem.keygen();
//!
//! // Bob uses pk3 to derive a key k and encapsulation c
//! let (c, k) = kem.encaps(&pk);
//!
//! // Bob sends c to Alice
//! // Alice uses s, c, sk3 and pk3 to recover k
//! let k_recovered = kem.decaps(&c, &sk);
//!
//! assert_eq!(k, k_recovered);
//! ```
//! For the PKE:
//!
//! ```rust
//! use kcimpl::{kyber512pke, ByteArray};
//! let pke = kyber512pke();
//!
//! // Bob wants to send an encrypted message to Alice
//! let m = ByteArray::random(32);
//! let r = ByteArray::random(32);
//!
//! // Alice runs keygen, publishes pk. Value sk is secret
//! let (sk, pk) = pke.keygen();
//!
//! // Bob uses the public key to encrypt the message
//! let enc = pke.encrypt(&pk, &m, r.clone());
//!
//! // Bob sends enc to Alice
//! // Alice uses the secret key to recover m
//! let dec = pke.decrypt(&sk, &enc);
//!
//! assert_eq!(m, dec);
//! ```
//! ///RSC TESTING
//! ```rust
//!  use kcimpl::{RSC};
//!  let mut rsc = RSC::new();
//!
//!   let input_vector = vec![1, 1, 0, 0, 1, 0, 1, 0, 1, 1];
//!   let (output_vector, _) = rsc.execute(input_vector.clone());
//!
//!   println!("\n--test_rsc_encoder--");
//!   println!("input_vector = {:?}", input_vector);
//!   println!("output_vector = {:?}", output_vector);
//!   println!("state = {:?}", rsc.registers);
//!
//!   assert_eq!(rsc.registers, vec![0; rsc.registers.len()]);
//!
//!
//! ```
//!
//! //! ///TURBO ENCODER TESTING
//! ```rust
//!  use kcimpl::{RSC, TurboEncoder};
//!  let interleaver = vec![8, 3, 7, 6, 9, 0, 2, 5, 1, 4];
//!         let mut turbo_encoder = TurboEncoder::new(interleaver);
//!
//!         let input_vector = vec![1, 1, 0, 0, 1, 0, 1, 0, 1, 1];
//!         let output_vector = turbo_encoder.execute(input_vector);
//!
//!         let expected_vector_1 = vec![1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0];
//!         let expected_vector_2 = vec![1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1];
//!
//!         println!("\n--test_turbo_encoder--");
//!         println!("output = {:?}", output_vector);
//!
//!         // Verifica che gli elementi alle posizioni 1, 4, 7... (1::3 in Python) corrispondano a `expected_vector_1`
//!         let output_vector_1: Vec<u8> = output_vector.iter().enumerate()
//!             .filter(|&(i, _)| i % 3 == 1)
//!             .map(|(_, &val)| val)
//!             .collect();
//!         println!("vector 1 encoding: {:?} out vs exp {:?}", output_vector_1, expected_vector_1);
//!         assert_eq!(output_vector_1, expected_vector_1);
//!
//!         // Verifica che gli elementi alle posizioni 2, 5, 8... (2::3 in Python) corrispondano a `expected_vector_2`
//!         let output_vector_2: Vec<u8> = output_vector.iter().enumerate()
//!             .filter(|&(i, _)| i % 3 == 2)
//!             .map(|(_, &val)| val)
//!             .collect();
//!         println!("vector 2 encoding: {:?} out vs exp {:?}", output_vector_2, expected_vector_2);
//!         assert_eq!(output_vector_2, expected_vector_2);
//! ```
//! TURBO SISO DECODER TESTING
//! ```rust
//!
//! use kcimpl::{TurboEncoder, AWGN, SISODecoder};
//!   let interleaver = vec![0; 10];
//!         let block_size = interleaver.len() + 2;
//!
//!         let mut encoder = TurboEncoder::new(interleaver.clone());
//!         let mut channel = AWGN::new(5.0);
//!         let mut decoder = SISODecoder::new(block_size);
//!
//!         let input_vector = vec![0, 1, 0, 1, 1, 0, 1, 0, 0, 0];
//!         let encoded_vector = encoder.execute(input_vector.clone());
//!
//!         // Partendo dal vettore di encoded_vector che è un Vec<u8>
//!          let mut channel_vector: Vec<u8> = encoded_vector.iter().map(|&x| x).collect();
//!
//!         // Ora puoi passare channel_vector come &[u8] a AWGN::convert_to_symbols
//!         let mut channel_vector_symbols: Vec<f64> = AWGN::convert_to_symbols(&channel_vector);
//!         channel_vector_symbols = channel.execute(&channel_vector_symbols);
//!
//!         let demultiplexed_vector = SISODecoder::demultiplex(channel_vector_symbols);
//!         let mut decoded_vector = decoder.execute(demultiplexed_vector);
//!
//!        // Convert the decoded vector into binary form as in the original test
//!          let binary_decoded_vector: Vec<u8> = decoded_vector.into_iter().map(|b| (b > 0.0) as u8).collect();
//!
//!         // Extract the bits from `encoded_vector` every 3rd element and convert it to `u8`
//!         let encoded_bits: Vec<u8> = encoded_vector.iter().cloned().step_by(3).collect();
//!
//!         // Now you can assert equality between `encoded_bits` and `binary_decoded_vector`
//!         assert_eq!(encoded_bits, binary_decoded_vector);
//!
//!          // Debug output for verification
//! println!("\n--test_siso_decoder--");
//! println!("input_vector = {:?}", input_vector);
//! println!("encoded_vector = {:?}", encoded_vector);
//! println!("decoded_vector = {:?}", binary_decoded_vector);
//! ```
//!
//! TURBO DECODER TESTING, CYCLE COMPLETED
//! ```rust
//! use kcimpl::{AWGN, TurboDecoder, TurboEncoder};
//! let interleaver = vec![9, 8, 5, 6, 2, 1, 7, 0, 3, 4];
//! let mut encoder = TurboEncoder::new(interleaver.clone());
//! let mut decoder = TurboDecoder::new(interleaver.clone(), 2, 16);
//!
//! let mut channel = AWGN::new(20.0);
//!
//! let input_vector: Vec<usize> = vec![1, 1, 0, 1, 1, 0, 1, 0, 1, 0];
//! let encoded_vector = encoder.execute(input_vector.clone());
//!
//! let channel_vector = AWGN::convert_to_symbols(&encoded_vector);
//! let noisy_channel_vector = channel.execute(&channel_vector);
//!
//! let decoded_vector: Vec<i32> = decoder.execute(noisy_channel_vector.clone())
//!             .iter().map(|&b| if b > 0.0 { 1 } else { 0 })
//!             .collect();
//!
//! println!("\n--test_turbo_decoder--");
//! println!("input_vector = {:?}", input_vector);
//! println!("encoded_vector = {:?}", encoded_vector);
//! println!("decoded_vector = {:?}", decoded_vector);
//! let encoded_bits: Vec<i32> = encoded_vector.iter().step_by(3).map(|&b| b as i32).collect();
//!
//!
//! assert_eq!(encoded_bits, decoded_vector);
//! ```

extern crate sha3;


// src/lib.rs


// Re-esporta AWGN, RSC e SISODecoder direttamente, se necessario
// pub use utils::awgn::AWGN;
// pub use utils::rsc::RSC;
// pub use utils::siso_decoder::SISODecoder;

mod functions;
mod kem;
mod pke;
mod structures;
mod turboc;

///TURBOC TEST RUNNING

pub use turboc::awgn::AWGN;
pub use turboc::rsc::RSC;
pub use turboc::siso_decoder::SISODecoder;
pub use turboc::turbo_encoder::TurboEncoder;
pub use turboc::turbo_decoder::TurboDecoder;










pub use structures::ByteArray;

use kem::KEM;
use pke::PKE;

/// Instantiate the Kyber 512 PKE with the appropriate parameters
pub const fn kyber512pke() -> PKE<256, 2> {
    PKE::<256, 2>::init(3329, 2, 10, 3)
}

/// Instantiate the Kyber 512 KEM with the appropriate parameters
pub const fn kyber512kem() -> KEM<256, 2> {
    KEM::<256, 2>::init(kyber512pke(), 178, 800, 1632, 738)
}

/// Instantiate the Kyber 768 PKE with the appropriate parameters
pub const fn kyber768pke() -> PKE<256, 3> {
    PKE::<256, 3>::init(3329, 2, 10, 4)
}

/// Instantiate the Kyber 768 KEM with the appropriate parameters
pub const fn kyber768kem() -> KEM<256, 3> {
    KEM::<256, 3>::init(kyber768pke(), 164, 1184, 2400, 1088)
}
