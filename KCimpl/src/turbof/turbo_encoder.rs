use crate::turbof::interleaver::Interleaver;
use crate::turbof::RSCEncoder::RSCEncoder;
use crate::turbof::utils::bits_to_symbols;

pub struct TurboEncoder {
    ls: usize,           // Lunghezza del blocco di dati
    interleaver: Interleaver, // Interleaver per la permutazione
}

impl TurboEncoder {
    /// Crea un nuovo TurboEncoder con la lunghezza del blocco e la permutazione
    pub fn new(ls: usize, permutation: Vec<usize>) -> Self {
        let interleaver = Interleaver::new(permutation);
        TurboEncoder { ls, interleaver }
    }

    /// Codifica un blocco di dati utilizzando due RSC encoder
    pub fn encode(&self, data: &[i32]) -> (Vec<i32>, Vec<f64>, Vec<f64>) {
        let rsc_encoder = RSCEncoder;

        // Codifica il primo encoder con i dati originali
        let (systematic1, parity1) = rsc_encoder.encode(data);

        let data_converted=bits_to_symbols(&data);

        // Permutazione dei dati per il secondo encoder
        let interleaved_data = self.interleaver.interleave(&data_converted);

        // Codifica il secondo encoder con i dati permutati
        let (systematic2, parity2) = rsc_encoder.encode(&interleaved_data);

        (systematic1, bits_to_symbols(&parity1), bits_to_symbols(&parity2))
    }
}