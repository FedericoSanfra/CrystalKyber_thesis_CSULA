use rand::Rng;
use crate::turbof::interleaver::Interleaver;
use crate::turbof::siso_decoder::SISO;
use crate::turbof::turbo_decoder::TurboDecoder;
use crate::turbof::turbo_encoder::TurboEncoder;
use crate::turbof::utils::{generate_pn_sequence, symbols_to_bits};
use crate::turbof::utils::bits_to_symbols;

pub struct TurboSimulation {
    interleaver: Vec<usize>,
    block_size: usize,
    simulation_length: usize,
    error_probability: f64,
    iterations: usize,
    rate: f64,
}

impl TurboSimulation {
    /// Creazione della simulazione con i parametri principali
    pub fn new(
        interleaver: Vec<usize>,
        block_size: usize,
        simulation_length: usize,
        error_probability: f64,
        iterations: usize,
        rate: f64,
    ) -> Self {
        TurboSimulation {
            interleaver,
            block_size,
            simulation_length,
            error_probability,
            iterations,
            rate,
        }
    }

    /// Esegue la simulazione
    pub fn run(&self) {
        let pn_sequence = generate_pn_sequence(self.simulation_length, 0);
        let num_blocks = self.simulation_length / self.block_size;
        let mut total_errors = 0;
        let M = self.simulation_length / self.block_size; // Calcola M

        // Crea un encoder Turbo con la permutazione specificata
        let turbo_encoder = TurboEncoder::new(self.block_size, self.interleaver.clone());

        // Crea un'istanza di SISO e un Interleaver per il decoder
        let siso = SISO; // Assicurati di avere un costruttore appropriato per SISO
        let interleaver = Interleaver::new(self.interleaver.clone());

        // Crea un TurboDecoder
        let turbo_decoder = TurboDecoder::new(siso, interleaver, self.iterations);

        for block_index in 0..num_blocks {
            // Prendi un blocco dalla sequenza
            let start = block_index * self.block_size;
            let end = start + self.block_size;
            let block = &pn_sequence[start..end];

            //let block_converted = bits_to_symbols(block);

            // Esegui la codifica con TurboEncoder
            let (systematic, parity1, parity2) = turbo_encoder.encode(&block);

            let systematic_converted = bits_to_symbols(&systematic);

            // Simula il canale con il blocco codificato (sistematici + parità)
            let received_block = self.simulate_channel(systematic_converted.clone());

            // Esegui la decodifica usando il TurboDecoder
            let decoded_block = turbo_decoder.decode(&received_block, &parity1, &parity2, &block);

            let decoded_block_converted = symbols_to_bits(decoded_block);

            // Conta gli errori
            let errors = self.count_errors(block, decoded_block_converted);
            total_errors += errors;

            println!("Blocchi completati: {}, Errori totali: {}", block_index + 1, total_errors);
        }

        // Calcolo e stampa del totale degli errori normalizzato su M, simile a 'toterr' in MATLAB
        let toterr = total_errors as f64 / M as f64;
        println!("Simulazione completata! Errori totali normalizzati (toterr): {}", toterr);
    }

    /// Simula il canale BSC con probabilità error_probability
    fn simulate_channel(&self, block: Vec<f64>) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        block
            .into_iter()
            .map(|bit| {
                if rng.gen::<f64>() < self.error_probability {
                    -1.0 * bit // Flip del bit
                } else {
                    bit
                }
            })
            .collect()
    }

    /// Conta gli errori tra input e output
    fn count_errors(&self, original: &[i32], decoded: Vec<i32>) -> usize {
        original
            .iter()
            .zip(decoded.iter())
            .filter(|(&o, &d)| o != d)
            .count()
    }
}
