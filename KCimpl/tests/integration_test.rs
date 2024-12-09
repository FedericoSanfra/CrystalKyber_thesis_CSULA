#[cfg(test)]
mod tests {
    use super::*;
    use kcimpl::kyber512kem; // Importa l'implementazione KEM di Kyber512
    use kcimpl::turbof::{RSCEncoder, turbo_encoder, turbo_decoder, utils, interleaver, siso_decoder, turbo_simulation};
    use rand::Rng;
    use kcimpl::turbof::turbo_simulation::TurboSimulation;

    #[test]
    fn test_turbo_simulation() {
        // Configurazione semplice del tests, come il codice MATLAB
        let interleaver = vec![0, 1, 2, 3, 4, 5, 6, 7]; // Esempio di interleaver semplice
        let block_size = 8; // Dimensione del blocco
        let simulation_length = 64; // Lunghezza totale della simulazione
        let error_probability = 0.1; // Probabilità di errore del canale BSC
        let iterations = 5; // Numero di iterazioni per la decodifica
        let rate = 1.0; // Tasso di trasmissione

        // Creazione della simulazione
        let simulation = TurboSimulation::new(
            interleaver.clone(),
            block_size,
            simulation_length,
            error_probability,
            iterations,
            rate,
        );

        println!("Avvio della simulazione...");
        simulation.run(); // Esegui la simulazione

        // In questo caso, stampiamo direttamente il risultato all'interno della funzione `run`
    }
}
