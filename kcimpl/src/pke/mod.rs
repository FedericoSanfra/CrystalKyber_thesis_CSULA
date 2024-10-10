//! Public Key Encryption
//!
//! Structure that handles all the parameters and functions required to perform the PKE

use crate::functions::{
    compress::*,
    encode::*,
    ntt::*,
    utils::{cbd, g, parse, prf, xof},
};
use crate::structures::{
    algebraics::{FiniteRing, RingModule},
    ByteArray, PolyMatrix3329, PolyVec3329,
};
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;

use std::fmt;
use std::error::Error;
use crate::pke::SRError::EmptyDataError;

// Define your custom Error enum
#[derive(Debug)]
pub enum SRError {
    EmptyDataError(String),
    ReedSolomonError(String),  // Error from Reed-Solomon library
    InvalidShardsSize(String),    // Error for invalid key sizes
    EncodingError(String),
    ReconstructionError(String),
    TooFewShardsPresent(String),
    TooManyShards(String),     // Error for invalid shard count
}


// Implement Display for the custom error
impl fmt::Display for SRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SRError::ReconstructionError(ref err) => write!(f, "Reconstruction error: {}", err),
            SRError::TooFewShardsPresent(ref err) => write!(f, "TooFewShardsPresent: {}", err),
            SRError::EncodingError(ref err)=> write!(f, "EncodingError: {}", err),
            SRError::EmptyDataError(ref err) => write!(f, "EmptyDataErrorr: {}", err),
            SRError::ReedSolomonError(ref err) => write!(f, "ReedSolomonError: {}", err),
            SRError::InvalidShardsSize(ref err) => write!(f, "InvalidShardsSize: {}", err),
            SRError::TooManyShards(ref err) => write!(f, "TooManyShards: {}", err),
        }
    }
}

// Implement the Error trait for error handling
impl Error for SRError {}



/// Default length used for XOF
const XOF_LEN: usize = 4000;


#[derive(Clone)]
pub struct PKE<const N: usize, const K: usize> {
    eta: usize,
    q: usize,
    du: usize,
    dv: usize,
}
///n=256 n'=9 q=3329 round 2 nist documentation Furthermore,
///throughout this document 
/// is always 2. The values of k, du and dv vary for
/// different security levels.
/// TODO SR IMPLEMENTING FOR DIFFERENT KEY SIZE 512 AND ...

impl<const N: usize, const K: usize> PKE<N, K> {

    // Modify the function to return a Result with your custom error
    pub fn encode_key_sr(&self, el: ByteArray, data_shards: usize, parity_shards: usize) -> Result<ByteArray, SRError> {
        // Check for validation of element and shards
        if el.data.is_empty() {
            return Err(SRError::EmptyDataError("The byte array data is empty".to_string()));
        }
        if parity_shards > data_shards || data_shards == 0 || parity_shards == 0 {
            return Err(SRError::InvalidShardsSize("Number of shards invalid".to_string()));
        }

        // Calcola la dimensione dei chunk
        let chunk_size = (el.data.len() + data_shards -1) / data_shards; // Utilizza il metodo ceiling


        // Crea gli chunks utilizzando il metodo chunks
        let mut data_chunks: Vec<Vec<u8>> = Vec::new();

        for chunk in el.chunks(chunk_size) {
            let mut chunk_vec = chunk.to_vec();
            // Se il chunk ha meno byte di chunk_size, riempilo con 0
            if chunk_vec.len() < chunk_size {
                chunk_vec.resize(chunk_size, 0); // Riempie con 0 fino alla dimensione chunk_size
            }
            data_chunks.push(chunk_vec); // Aggiunge il chunk (riempito o no) alla lista dei data_chunks
        }

        if el.data.len() % data_shards != 0 {
            println!(" data chunks: {:?}", data_chunks);
        }

        // Crea un array di byte per gli shard
        let mut shards: Vec<Vec<u8>> = vec![vec![0u8; chunk_size]; data_shards + parity_shards];
        println!("shards {:?}", shards);

        // Popola i data shards
        for (i, chunk) in data_chunks.into_iter().enumerate() {
            if i < data_shards {
                shards[i].copy_from_slice(&chunk); // Copia il chunk nello shard corrispondente
            }
        }
        println!("shards filled situation {:?}", shards);

        // Codifica gli shard usando il metodo encode
        let reed_solomon = ReedSolomon::new(data_shards, parity_shards)
            .map_err(|_| SRError::InvalidShardsSize("Failed to create Reed-Solomon codec".to_string()))?;

        reed_solomon.encode(&mut shards)
            .map_err(|_| SRError::EncodingError("Failed to encode shards".to_string()))?;

        // Flatten the shards into a single ByteArray
        let encoded_data: Vec<u8> = shards.into_iter().flat_map(|s| s).collect();

        println!("Encoded data: {:?}", encoded_data);


        Ok(ByteArray::from_bytes(&encoded_data))
    }

    ///reconstruction algorithm Solomon-Reed
    /// Reconstruction algorithm Solomon-Reed with verification
    pub fn reconstruct_key_sr(
        &self,
        mut encoded_data: Vec<Option<Vec<u8>>>, // Cambiato a Vec<Option<Vec<u8>>>, perchè è un passaggio diretto
        data_shards: usize,
        parity_shards: usize,
    ) -> Result<ByteArray, SRError> {
        // Controllo input
        if encoded_data.is_empty() {
            return Err(SRError::EmptyDataError("The encoded data is empty".to_string()));
        }
        if parity_shards > data_shards || data_shards == 0 || parity_shards == 0 {
            return Err(SRError::InvalidShardsSize("Invalid number of shards".to_string()));
        }

        // Crea il codec Reed-Solomon
        let reed_solomon = ReedSolomon::new(data_shards, parity_shards)
            .map_err(|_| SRError::InvalidShardsSize("Failed to create Reed-Solomon codec".to_string()))?;

        // Prima verifica la validità dei dati
        let valid_shards: Vec<&[u8]> = encoded_data
            .iter()
            .filter_map(|shard| shard.as_deref()) // Trasforma Option<Vec<u8>> in &[u8]
            .collect();

        if reed_solomon.verify(&valid_shards).unwrap_or(false) {
            println!("I dati sono validi, nessuna ricostruzione necessaria.");
            // I dati sono validi, ritorna i data shards originali
            let mut original_data = Vec::new();
            for shard in &encoded_data[..data_shards] {
                if let Some(data) = shard {
                    original_data.extend(data);
                } else {
                    return Err(SRError::ReconstructionError("Missing data shard during verification".to_string()));
                }
            }

            // Rimuovi eventuali padding alla fine
            let data_len = original_data.len() - original_data.iter().rev().take_while(|&&b| b == 0).count();
            original_data.truncate(data_len);

            return Ok(ByteArray::from_bytes(&original_data));
        }

        // Se la verifica fallisce, tenta di ricostruire i dati
        println!("Dati non validi, tentativo di ricostruzione...");

        // Ricostruisce i dati mancanti
        reed_solomon.reconstruct(&mut encoded_data)
            .map_err(|_| SRError::ReconstructionError("Failed to reconstruct shards".to_string()))?;

        println!("Shards after reconstruction: {:?}", encoded_data);

        // Estrai i data shards ricostruiti
        let mut reconstructed_data = Vec::new();
        for shard in &encoded_data[..data_shards] {
            if let Some(data) = shard {
                reconstructed_data.extend(data);
            } else {
                return Err(SRError::ReconstructionError("Missing data shard during reconstruction".to_string()));
            }
        }

        println!("Reconstructed data: {:?}", reconstructed_data);

        // Rimuovi eventuali padding alla fine
        let data_len = reconstructed_data.len() - reconstructed_data.iter().rev().take_while(|&&b| b == 0).count();
        reconstructed_data.truncate(data_len);
        println!("Padding removed: {:?}", reconstructed_data);

        // Converte in ByteArray usando il metodo from_bytes
        Ok(ByteArray::from_bytes(&reconstructed_data))
    }



    /// Kyber CPAPKE Key Generation => (secret key, public key)
    /// Algorithm 4 p. 9
    pub fn keygen(&self) -> (ByteArray, ByteArray) {
        let d = ByteArray::random(32);
        let (rho, sigma) = g(&d);

        let mut a = PolyMatrix3329::init();

        for i in 0..K {
            for j in 0..K {
                a.set(i, j, parse(&xof(&rho, j, i, XOF_LEN), self.q));
            }
        }

        let (mut s, mut e) = (PolyVec3329::<N, K>::init(), PolyVec3329::<N, K>::init());
        let prf_len = 64 * self.eta;

        for i in 0..K {
            s.set(i, cbd(prf(&sigma, i, prf_len), self.eta));
            e.set(i, cbd::<N>(prf(&sigma, K + i, prf_len), self.eta));
        }
        let s_hat = ntt_vec(&s);
        let e_hat = ntt_vec(&e);

        let t_hat = bcm_matrix_vec(&a, &s_hat).add(&e_hat);

        let pk = encode_polyvec(t_hat, 12).append(&rho);
        let sk = encode_polyvec(s_hat, 12);

        (sk, pk)
    }
    ///cbd is a discrete version of normal distribution, used to keep the variance
/// in the 2*eta range
///bcm_matrix_vec function to be investigated more

    /// Kyber CPAPKE Encryption : public key, message, random coins => ciphertext
    /// Algorithm 5 p. 10
    pub fn encrypt(&self, pk: &ByteArray, m: &ByteArray, r: ByteArray) -> ByteArray {
        let offset = 12 * K * N / 8;
        let prf_len = 64 * self.eta;

        let (t, rho) = pk.split_at(offset);
        let t_hat = decode_to_polyvec(t, 12);
        let mut a_t = PolyMatrix3329::init();

        for i in 0..K {
            for j in 0..K {
                a_t.set(i, j, parse(&xof(&rho, i, j, XOF_LEN), self.q));
            }
        }

        let (mut r_bold, mut e1) = (PolyVec3329::<N, K>::init(), PolyVec3329::<N, K>::init());
        for i in 0..K {
            r_bold.set(i, cbd(prf(&r, i, prf_len), self.eta));
            e1.set(i, cbd(prf(&r, K + i, prf_len), self.eta));
        }
        let e2 = cbd(prf(&r, 2 * K, prf_len), self.eta);

        let r_hat = ntt_vec(&r_bold);
        let u_bold = ntt_product_matvec(&a_t, &r_hat).add(&e1);

        let v = ntt_product_vec(&t_hat, &r_hat)
            .add(&e2)
            .add(&decompress_poly(
                decode_to_poly::<N>(m.clone(), 1),
                1,
                self.q,
            ));

        let c1 = encode_polyvec(compress_polyvec(u_bold, self.du, self.q), self.du);
        let c2 = encode_poly(compress_poly(v, self.dv, self.q), self.dv);

        c1.append(&c2)
    }
    ///generation of encrypted message in chipertext, compressed (!!)

    /// Kyber CPAPKE Decryption : secret key, ciphertext => message
    /// Algorithm 6 p. 10
    pub fn decrypt(&self, sk: &ByteArray, c: &ByteArray) -> ByteArray {
        let offset = self.du * K * N / 8;
        let (c1, c2) = c.split_at(offset);

        let u = decompress_polyvec(decode_to_polyvec::<N, K>(c1, self.du), self.du, self.q);
        let v = decompress_poly(decode_to_poly(c2, self.dv), self.dv, self.q);
        let s = decode_to_polyvec(sk.clone(), 12);

        let u_hat = ntt_vec(&u);
        let x = ntt_product_vec(&s, &u_hat);
        let p = v.sub(&x);

        encode_poly(compress_poly(p, 1, self.q), 1)
    }

    pub const fn init(q: usize, eta: usize, du: usize, dv: usize) -> Self {
        Self { q, eta, du, dv }
    }
}

#[test]
fn pke_keygen_cpapke_512() {
    let pke = crate::kyber512pke();
    pke.keygen();
}

#[test]
fn pke_keygen_cpapke_768() {
    let pke = crate::kyber768pke();
    pke.keygen();
}

#[test]
fn encrypt_then_decrypt_cpapke_512() {
    let pke = crate::kyber512pke();
    let (sk, pk) = pke.keygen();

    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let enc = pke.encrypt(&pk, &m, r);
    let dec = pke.decrypt(&sk, &enc);

    assert_eq!(m, dec);
}

#[test]
fn encrypt_then_decrypt_cpapke_768() {
    let pke = crate::kyber768pke();
    let (sk, pk) = pke.keygen();

    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let enc = pke.encrypt(&pk, &m, r);
    let dec = pke.decrypt(&sk, &enc);

    assert_eq!(m, dec);
} //to finish to read this  and decrypt

///encoding test suite Solomon Reed

#[cfg(test)]
mod encoding_tests {
    use super::*;

    #[test]
    fn test_valid_encoding() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave valida, data_shards e parity_shards corretti
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);  // 10 byte
        let data_shards = 5;
        let parity_shards = 3;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key.clone(), data_shards, parity_shards);
        //println!("key original: {:?}", key);
        // Verifica che la codifica abbia successo
        assert!(result.is_ok(), "Expected successful encoding, but got an error.");
        let encoded_key = result.unwrap();
        println!(" encoded key: {:?}", encoded_key);

        // Verifica che il risultato abbia la dimensione attesa
        let chunk_size = (&key.data.len() + data_shards - 1) / data_shards;
        let expected_len = (data_shards + parity_shards) * chunk_size;
        assert_eq!(encoded_key.data.len(), expected_len, "Encoded data length mismatch.");
    }

    #[test]
    fn test_empty_byte_array() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con un ByteArray vuoto
        let key = ByteArray::new();  // Chiave vuota
        let data_shards = 3;
        let parity_shards = 2;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che ritorni l'errore corretto per chiave vuota
        assert!(matches!(result, Err(SRError::EmptyDataError(_))), "Expected EmptyDataError.");
    }

    #[test]
    fn test_invalid_shard_sizes() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con parametri di shard non validi
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5]);  // 5 byte
        let data_shards = 2;
        let parity_shards = 3;  // Numero di parity_shards > data_shards

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che ritorni l'errore per shard non validi
        assert!(matches!(result, Err(SRError::InvalidShardsSize(_))), "Expected InvalidShardsSize error.");
    }

    #[test]
    fn test_zero_data_shards() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con zero data_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5]);  // 5 byte
        let data_shards = 0;
        let parity_shards = 3;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che ritorni l'errore per data_shards = 0
        assert!(matches!(result, Err(SRError::InvalidShardsSize(_))), "Expected InvalidShardsSize error for data_shards=0.");
    }

    #[test]
    fn test_zero_parity_shards() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con zero parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5]);  // 5 byte
        let data_shards = 3;
        let parity_shards = 0;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che ritorni l'errore per parity_shards = 0
        assert!(matches!(result, Err(SRError::InvalidShardsSize(_))), "Expected InvalidShardsSize error for parity_shards=0.");
    }

    #[test]
    fn test_encoding_with_exact_shards() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con una dimensione della chiave divisibile esattamente per il numero di data_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6]);  // 6 byte
        let data_shards = 3;  // Divisibile esattamente
        let parity_shards = 2;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che la codifica abbia successo
        assert!(result.is_ok(), "Expected successful encoding, but got an error.");
    }

    #[test]
    fn test_encoding_with_non_divisible_shards() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con una dimensione della chiave non divisibile esattamente per il numero di data_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7]);  // 7 byte
        let data_shards = 3;  // Non divisibile esattamente
        let parity_shards = 2;

        // Chiama il metodo encode_key_sr
        let result = pke.encode_key_sr(key, data_shards, parity_shards);

        // Verifica che la codifica abbia successo
        assert!(result.is_ok(), "Expected successful encoding, but got an error.");
    }
}

#[cfg(test)]
mod decoding_tests {
    use super::*;

    #[test]
    fn test_valid_reconstruction_with_loss() {
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]); // 10 byte
        let data_shards = 5;
        let parity_shards = 3;

        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di 3 shards (2 data shards e 1 parity shard)
        shards[0] = None; // Perdita del primo data shard
        shards[5] = None; // Perdita del sesto shard (parity shard)
        shards[3] = None; // Perdita di un altro data shard

        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        assert!(result.is_ok(), "Expected successful reconstruction");
        let reconstructed_key = result.unwrap();
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch");
    }

    #[test]
    fn test_reconstruction_with_different_key_lengths() {
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Chiave più lunga (20 byte)
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        let data_shards = 5;
        let parity_shards = 3;

        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di 2 shards
        shards[1] = None; // Perdita del secondo data shard
        shards[6] = None; // Perdita di un parity shard

        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        assert!(result.is_ok(), "Expected successful reconstruction");
        let reconstructed_key = result.unwrap();
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch");
    }

    #[test]
    fn test_reconstruction_with_corrupted_shards() {
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let data_shards = 5;
        let parity_shards = 3;

        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Corrompi due shards modificandone i valori
        shards[2] = Some(vec![0, 0, 0, 0]); // Shard corrotto
        shards[6] = Some(vec![0, 0, 0, 0]); // Parity shard corrotto

        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        assert!(result.is_err(), "Expected reconstruction failure due to corrupted shards");
    }

    #[test]
    fn test_reconstruction_without_loss() {
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let data_shards = 5;
        let parity_shards = 3;

        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        let shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Nessuna perdita di shard
        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        assert!(result.is_ok(), "Expected successful reconstruction with no loss");
        let reconstructed_key = result.unwrap();
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch");
    }

    #[test]
    fn test_reconstruction_with_excessive_loss() {
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let data_shards = 5;
        let parity_shards = 3;

        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di 4 shards, che è oltre il limite di tolleranza
        shards[0] = None; // Perdita del primo shard
        shards[1] = None; // Perdita del secondo shard
        shards[4] = None; // Perdita di un altro shard
        shards[6] = None; // Perdita di un parity shard

        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        assert!(result.is_err(), "Expected reconstruction failure due to excessive loss");
    }
    
    #[test]
    fn test_reconstruction_with_different_shard_count() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave di lunghezza 15 byte, 6 data_shards e 4 parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);  // 15 byte
        let data_shards = 6;
        let parity_shards = 4;

        // Codifica la chiave
        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();
        println!("Encoded key: {:?}", encoded_key);

        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        // Crea un array di shards da ricostruire
        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di 3 shards:
        // Perdita del secondo data shard (posizione 1)
        shards[1] = None;

        // Perdita di due parity shard (posizioni 7 e 9)
        shards[7] = None;
        shards[9] = None;

        println!("Before reconstruction: {:?}", shards);

        // Chiama il metodo reconstruct_key_sr, passando gli shards con i None
        let result = pke.reconstruct_key_sr(
            shards, // Passa gli shards direttamente
            data_shards,
            parity_shards
        );

        // Verifica che la ricostruzione abbia successo, poiché ci sono ancora sufficienti parity shards per recuperare i dati
        assert!(result.is_ok(), "Expected successful reconstruction, but got an error.");
        let reconstructed_key = result.unwrap();

        // Verifica che il risultato corrisponda alla chiave originale
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch.");
    }

    #[test]
    fn test_reconstruction_with_all_parity_shards_missing() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave di lunghezza 20 byte, 5 data_shards e 3 parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        let data_shards = 5;
        let parity_shards = 3;

        // Codifica la chiave
        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();

        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        // Crea un array di shards
        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di tutti i parity shards (posizioni da 5 a 7)
        shards[5] = None;
        shards[6] = None;
        shards[7] = None;

        // Chiama il metodo di ricostruzione
        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);

        // Verifica che la ricostruzione abbia successo (tutti i data shards sono presenti)
        assert!(result.is_ok(), "Expected successful reconstruction with all parity shards missing.");
        let reconstructed_key = result.unwrap();
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch.");
    }


    #[test]
    fn test_reconstruction_with_all_data_shards_missing() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave di lunghezza 20 byte, 5 data_shards e 3 parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        let data_shards = 5;
        let parity_shards = 3;

        // Codifica la chiave
        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();

        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        // Crea un array di shards
        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di tutti i data shards (posizioni da 0 a 4)
        shards[0] = None;
        shards[1] = None;
        shards[2] = None;
        shards[3] = None;
        shards[4] = None;

        // Chiama il metodo di ricostruzione
        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);

        // Verifica che la ricostruzione fallisca, poiché non ci sono data shards sufficienti
        assert!(result.is_err(), "Expected reconstruction to fail with all data shards missing.");
    }
    //test with corrupted shards is not making sense here, I'll implement something later on
    #[test]
    fn test_reconstruction_with_random_loss() {
        use rand::Rng;

        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave di lunghezza 20 byte, 6 data_shards e 4 parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
        let data_shards = 6;
        let parity_shards = 4;

        // Codifica la chiave
        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();

        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        // Crea un array di shards
        let mut shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Genera un RNG per perdere casualmente 3 shards
        let mut rng = rand::thread_rng();
        let mut loss_indices = Vec::new();
        while loss_indices.len() < 3 {
            let idx = rng.gen_range(0..(data_shards + parity_shards));
            if !loss_indices.contains(&idx) {
                shards[idx] = None; // Simula la perdita dello shard
                loss_indices.push(idx);
            }
        }

        println!("Shards lost at positions: {:?}", loss_indices);

        // Chiama il metodo di ricostruzione
        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);

        // Verifica che la ricostruzione abbia successo
        assert!(result.is_ok(), "Expected successful reconstruction with random loss.");
        let reconstructed_key = result.unwrap();
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch.");
    }


    #[test]
    fn test_no_reconstruction_needed_when_all_shards_present() {
        // Inizializza un'istanza di PKE
        let pke = PKE::<256, 2>::init(3329, 2, 10, 4);

        // Test con chiave di lunghezza 12 byte, 4 data_shards e 2 parity_shards
        let key = ByteArray::from_bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let data_shards = 4;
        let parity_shards = 2;

        // Codifica la chiave
        let encoded_key = pke.encode_key_sr(key.clone(), data_shards, parity_shards).unwrap();

        let chunk_size = encoded_key.data.len() / (data_shards + parity_shards);

        // Crea un array di shards (tutti presenti)
        let shards: Vec<Option<Vec<u8>>> = encoded_key.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Chiama il metodo di ricostruzione
        let result = pke.reconstruct_key_sr(shards.clone(), data_shards, parity_shards);

        // Verifica che la ricostruzione restituisca un successo senza necessità di ricostruire (shards già intatti)
        assert!(result.is_ok(), "Expected successful verification without need for reconstruction.");

        let reconstructed_key = result.unwrap();

        // Verifica che la chiave ricostruita sia identica a quella originale
        assert_eq!(reconstructed_key.data, key.data, "Reconstructed data mismatch.");
    }


}
