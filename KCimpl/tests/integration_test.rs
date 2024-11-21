#[cfg(test)]
mod tests {

    use kcimpl::kyber512kem; // Importa l'implementazione KEM di Kyber512

    #[test]
    fn test_kem_encapsulation_decapsulation_sr() {
        // Step 1: Inizializzazione del KEM (Kyber512)
        let kem = kyber512kem();

        // Step 2: Alice genera la coppia di chiavi (chiave segreta e pubblica)
        let (sk, pk) = kem.keygen();
        println!("Chiave segreta (sk): {:?}", sk);
        println!("Chiave pubblica (pk): {:?}", pk);

        // Step 3: Bob usa la chiave pubblica di Alice per generare la chiave condivisa (k) e il ciphertext (c)
        let (c, k) = kem.encaps(&pk);
        println!("Ciphertext (c): {:?}", c);
        println!("Chiave condivisa (k): {:?}", k);

        // Step 3.5: Prima di inviare il cyphertext viene codificato con Solomon Reed

        let encoded_data=kem.encoding_sr(c, 16, 6);
        //perdita dati simulata nel metodo kem.decoding

        println!("Encoded data in Solomon Reed with loss: {:?}", encoded_data);
        // Step 4: Bob invia il ciphertext (c) ad Alice

        //Step 4.5: Il cyphertext viene ricostruito tramite Solomon Reed

        let reconstructed_data=kem.decoding_sr(encoded_data, 16, 6);
        println!("Reconstructed data with Solomon Reed with loss simulated: {:?}", reconstructed_data);

        // Step 5: Alice usa la sua chiave segreta (sk) e il ciphertext ricevuto (c) per recuperare la chiave condivisa (k_recovered)
        let k_recovered = kem.decaps(&reconstructed_data, &sk);
        println!("Chiave condivisa recuperata da Alice (k_recovered): {:?}", k_recovered);

        // Step 6: Verifica che la chiave condivisa di Bob (k) sia uguale alla chiave recuperata da Alice (k_recovered)
        assert_eq!(k, k_recovered, "La chiave condivisa non coincide con quella recuperata!");
    }
    ///512 PKE TEST SOLOMON-REED ENCODING
    use super::*;
    use kcimpl::{kyber512pke, ByteArray};

    #[test]
    fn test_pke_encryption_decryption_sr() {
        let pke = kyber512pke();

        // Bob wants to send an encrypted message to Alice
        let m = ByteArray::random(32);
        let r = ByteArray::random(32);

        // Alice runs keygen, publishes pk. Value sk is secret
        let (sk, pk) = pke.keygen();
        println!("sk-> {:?}", sk);
        println!("pk-> {:?}", pk);



        // Bob uses the public key to encrypt the message
        let enc = pke.encrypt(&pk, &m, r.clone());
        println!("enc: {:?}", enc);
        println!("message original: {:?}", m);
        let data_shards = 16;
        let parity_shards = 6;

        let encoded_message= pke.encode_key_sr(enc.clone(),data_shards,parity_shards);

        let encoded=encoded_message.unwrap();

        let chunk_size = encoded.data.len() / (data_shards + parity_shards);

        ///SK DATA SHARDS LOSS SIMULATION
        let mut shards: Vec<Option<Vec<u8>>> = encoded.data.chunks(chunk_size)
            .map(|chunk| Some(chunk.to_vec()))
            .collect();

        // Simula la perdita di 3 shards (2 data shards e 1 parity shard)
        shards[0] = None; // Perdita del primo data shard
        shards[5] = None; // Perdita del sesto shard (parity shard)
        shards[3] = None; // Perdita di un altro data shard

        let result = pke.reconstruct_key_sr(shards, data_shards, parity_shards);
        let reconstructed_message = result.unwrap();

        // Bob sends enc to Alice and
        // Alice uses the secret key to recover m
        let dec = pke.decrypt(&sk, &reconstructed_message);
        println!("decrypted message: {:?}", dec);
        ///The sk stays in Alice system, while pk is published
        /// I'm not considering pk publishing system in this test
        assert_eq!(m, dec);
    }
}
