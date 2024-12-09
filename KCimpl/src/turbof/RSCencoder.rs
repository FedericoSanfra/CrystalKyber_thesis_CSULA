pub struct RSCEncoder;

impl RSCEncoder {
    /// Codifica un blocco di dati con RSC, inclusa la terminazione del trellis.
    pub fn encode(&self, data: &[i32]) -> (Vec<i32>, Vec<i32>) {
        let mut state = [0, 0]; // Stato iniziale
        let mut parity = vec![];
        let mut systematic = data.to_vec();

        // Codifica del blocco di dati
        for &bit in data {
            let parity_bit = (state[0] ^ state[1] ^ bit) & 1;
            state[1] = state[0];
            state[0] = bit;
            parity.push(parity_bit);
        }

        // Terminazione del trellis: assicura che lo stato ritorni a zero
        for _ in 0..2 {
            let parity_bit = (state[0] ^ state[1]) & 1;
            state[1] = state[0];
            state[0] = 0; // Forza lo stato a zero
            parity.push(parity_bit);
            systematic.push(0); // Aggiunge bit sistematici per la terminazione
        }

        (systematic, parity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsc_encoder() {
        let encoder = RSCEncoder;
        let data = vec![1, 0, 1, 1];
        let (systematic, parity) = encoder.encode(&data);

        // Controlla i risultati
        assert_eq!(systematic, vec![1, 0, 1, 1, 0, 0]); // Dati più terminazione
        assert_eq!(parity.len(), 6); // Lunghezza pari a systematic
    }
}
