//! This is documentation for the `kcimpl` crate.
//!
//! # Introduction
//! `kcimpl` is an implementation of Crystals-Kyber , a post-quantum
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
//! println!(" pk length {:?}", pk.data.len());
//!
//! // Bob uses pk3 to derive a key k and encapsulation c
//! let (c, k) = kem.encaps(&pk);
//!
//! // Bob sends c to Alice
//! // Alice uses s, c, sk3 and pk3 to recover k
//!
//! let k_recovered = kem.decaps(&c, &sk);
//! //debug_assert_eq!(1380, c.data.len());
//! //736
//! //800
//! assert_eq!(k, k_recovered);
//! ```
//! For measurements and metrics:
//!
//! For the PKE:
//! ```rust
//! use kcimpl::kyber512kem;
//! use std::time::Instant;
//! use sysinfo::{System, Process, Pid};
//!
//! fn main() {
//!     let mut sys = System::new_all();
//!     sys.refresh_all();
//!
//!     let kem = kyber512kem();
//!
//!     // Registra il tempo di inizio
//!     let start_time = Instant::now();
//!
//!     // Registra l'uso della CPU e memoria prima dell'esecuzione
//!     let process_id = sysinfo::get_current_pid().unwrap();
//!     let mut cpu_before = 0.0;
//!     let mut mem_before = 0;
//!
//!     if let Some(proc) = sys.process(process_id) {
//!         cpu_before = proc.cpu_usage();
//!         mem_before = proc.memory();
//!     }
//!
//!     // Alice runs keygen
//!     let (sk, pk) = kem.keygen();
//!     println!("Public key length: {:?}", pk.data.len());
//!
//!     // Bob encapsulates key
//!     let (c, k) = kem.encaps(&pk);
//!
//!     // Alice decapsulates key
//!     let k_recovered = kem.decaps(&c, &sk);
//!
//!     // Registra il tempo di fine
//!     let duration = start_time.elapsed();
//!
//!     // Aggiorna info di sistema dopo l'esecuzione
//!     sys.refresh_all();
//!     let mut cpu_after = 0.0;
//!     let mut mem_after = 0;
//!
//!     if let Some(proc) = sys.process(process_id) {
//!         cpu_after = proc.cpu_usage();
//!         mem_after = proc.memory();
//!     }
//!
//!     // Verifica la correttezza della decapsulazione
//!     assert_eq!(k, k_recovered);
//!
//!     // Stampa delle metriche di performance
//!     println!("Execution time: {:?}", duration);
//!     println!("CPU usage: {:.2}%", cpu_after - cpu_before);
//!     println!("Memory usage (bytes): {}", mem_after - mem_before);
//! }
//! ```
//!
//! ```rust
//! use std::fs::OpenOptions;
//! use std::io::Write;
//! use kcimpl::{kyber512pke, ByteArray};
//! let pke = kyber512pke();
//! let output_file_path = "C:\\Users\\feder\\RustroverProjects\\TurboCodesRust_thesis\\kcimpl\\src\\kyber_keys.txt";
//!         let mut output_file = OpenOptions::new()
//!            .create(true)
//!             .write(true)
//!             .append(true)
//!             .open(output_file_path)
//!             .expect("Unable to open or create output file");
//!
//! for i in 0..200{
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
//! let output_line = format!(
//!                 "{:?}",
//!                 enc.data
//!             );
//! output_file.write_all(output_line.as_bytes()).expect("Failed to write to output file");
//! //assert_eq!(736, enc.data.len());
//!
//! // Bob sends enc to Alice
//! // Alice uses the secret key to recover m
//! let dec = pke.decrypt(&sk, &enc);
//!
//! }
//!
//! //assert_eq!(m, dec);
//! ```

extern crate sha3;

mod functions;
mod kem;
mod pke;
mod structures;
pub mod turbof;

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
