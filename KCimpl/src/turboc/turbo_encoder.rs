use std::vec::Vec;

use crate::turboc::rsc::RSC;

#[derive(Clone)]
pub struct TurboEncoder {
    interleaver: Vec<usize>,
    block_size: usize,
    encoders: Vec<RSC>,  // Supponiamo che RSC sia una struttura già definita in Rust
}

impl TurboEncoder {
    // Costruttore, riceve come parametro il vettore di usize, gli indici del vettore che subisce l'interleaver
    pub fn new(interleaver: Vec<usize>) -> TurboEncoder {
        let block_size = interleaver.len();
        let encoders = vec![RSC::new(), RSC::new()];  // Supponiamo che RSC abbia un costruttore `new()`

        TurboEncoder {
            interleaver,
            block_size,
            encoders,
        }
    }

    // Reset degli encoder
    pub fn reset(&mut self) {
        for encoder in &mut self.encoders {
            encoder.reset();  // Assumiamo che RSC abbia un metodo `reset()`
        }
    }

    // Funzione di interleaving, riceve come parametro il vettore di input, usize per il momento
    pub fn interleave(&self, vector: Vec<usize>) -> Vec<usize> {
        let mut interleaved = vec![0; self.block_size];

        for i in 0..self.block_size {
            interleaved[i] = vector[self.interleaver[i]];
        }

        interleaved
    }

    // Funzione di esecuzione (encoding)
    pub fn execute(&mut self, vector: Vec<usize>) -> Vec<u8> {
        let output_size = 3 * (vector.len() + self.encoders[0].registers.len());
        //TODO viene aggiunto due come i registri, ma se voglio aumentare i tail bits???
        let mut output = vec![0; output_size];
        let interleaved = self.interleave(vector.clone());

        // Encoder 0
        let (output_0, output_1) = self.encoders[0].execute(vector.clone());

        // Inserimento di `output_1` nelle posizioni `0, 3, 6, ...`
        for (i, &val) in output_1.iter().enumerate() {
            output[i * 3] = val;
        }

        // Inserimento di `output_0` nelle posizioni `1, 4, 7, ...`
        for (i, &val) in output_0.iter().enumerate() {
            output[i * 3 + 1] = val;
        }
         let commit=0;

        // Encoder 1
        let (output_1, _) = self.encoders[1].execute(interleaved);
        for (i, &val) in output_1.iter().enumerate() {
            output[i * 3 + 2] = val; // Posizioni per Encoder 1 (partenza da 2)
        }

        output
    } // commit
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turbo_encoder_creation() {
        let interleaver = vec![2, 0, 1];
        let encoder = TurboEncoder::new(interleaver.clone());

        assert_eq!(encoder.block_size, interleaver.len());
        assert_eq!(encoder.interleaver, interleaver);
        assert_eq!(encoder.encoders.len(), 2); // Due encoder RSC
    }

    #[test]
    fn test_reset() {
        let interleaver = vec![0, 1, 2];
        let mut encoder = TurboEncoder::new(interleaver);

        // Eseguiamo il reset per verificare che non causi errori
        encoder.reset();

        // Supponiamo che il reset di ciascun encoder RSC azzeri il suo stato
        // Qui ci assicuriamo semplicemente che il metodo funzioni senza panico
    }

    #[test]
    fn test_interleave() {
        let interleaver = vec![2, 0, 1];
        let encoder = TurboEncoder::new(interleaver);

        let input_vector = vec![10, 20, 30];
        let interleaved_vector = encoder.interleave(input_vector);

        assert_eq!(interleaved_vector, vec![30, 10, 20]); // Verifica che l'interleaving sia corretto
    }

}
