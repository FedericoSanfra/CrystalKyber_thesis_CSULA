use rand::prelude::*;
use rand_distr::StandardNormal;

pub struct AWGN {
    scale: f64,
}

impl AWGN {
    /// Converte un vettore binario (0, 1) in simboli (-1, +1).
    pub fn convert_to_symbols(vector: &[u8]) -> Vec<f64> {
        vector.iter().map(|&bit| (bit as f64) * 2.0 - 1.0).collect()
    }

    /// Crea una nuova istanza di AWGN con un dato livello di rumore in dB.
    pub fn new(noise_db: f64) -> Self {
        let scale = 1.0 / (10.0f64.powf(noise_db / 20.0));
        AWGN { scale }
    }
//da modificare BSC
    /// Applica il rumore al vettore.
    pub fn execute(&self, vector: &[f64]) -> Vec<f64> {
        let mut rng = thread_rng();
        vector
            .iter()
            .map(|&v| {
                let noise: f64 = rng.sample(StandardNormal); // Rumore normale standard
                v + noise * self.scale
            })
            .collect()
    }
}

// fn main() {
//     // Esempio di utilizzo
//     let awgn = AWGN::new(10.0); // 10 dB di rumore
//     let input_vector = vec![0, 1, 0, 1, 1]; // Vettore di esempio
//
//     let symbols = AWGN::convert_to_symbols(&input_vector); // Conversione in simboli
//     let noisy_output = awgn.execute(&symbols); // Aggiunta del rumore
//
//     println!("Input: {:?}", input_vector);
//     println!("Symbols: {:?}", symbols);
//     println!("Noisy Output: {:?}", noisy_output);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_symbols() {
        // Test base di conversione binaria in simboli
        let input_vector = vec![0, 1, 0, 1];
        let symbols = AWGN::convert_to_symbols(&input_vector);
        let expected_symbols = vec![-1.0, 1.0, -1.0, 1.0];
        assert_eq!(symbols, expected_symbols);
    }

    #[test]
    fn test_convert_to_symbols_empty() {
        // Test di un vettore vuoto: dovrebbe ritornare un array vuoto
        let input_vector: Vec<u8> = vec![];
        let symbols = AWGN::convert_to_symbols(&input_vector);
        assert!(symbols.is_empty());
    }

    #[test]
    fn test_execute_with_noise() {
        // Test base: aggiungere rumore a un vettore di simboli e controllare che ci siano cambiamenti
        let awgn = AWGN::new(10.0); // 10 dB di rumore
        let symbols = vec![-1.0, 1.0, -1.0, 1.0, 1.0];
        let noisy_output = awgn.execute(&symbols);

        // Assicurarsi che la dimensione del risultato corrisponda all'input
        assert_eq!(noisy_output.len(), symbols.len());

        // Controllo che il rumore abbia effettivamente introdotto variazioni
        let changed = noisy_output
            .iter()
            .zip(symbols.iter())
            .any(|(&noisy, &original)| (noisy - original).abs() > 0.1); // Un cambiamento evidente
        assert!(changed, "Il rumore non ha introdotto variazioni evidenti");
    }

    #[test]
    fn test_execute_high_noise() {
        // Test con alto rumore (0 dB), dovrebbe introdurre variazioni più significative
        let awgn = AWGN::new(0.0); // Massimo rumore
        let symbols = vec![-1.0, 1.0, -1.0, 1.0, 1.0];
        let noisy_output = awgn.execute(&symbols);

       // println!("{:?}", noisy_output);
        // Controlliamo che ogni simbolo differisca significativamente dal suo valore originale
        let changed = noisy_output
            .iter()
            .zip(symbols.iter())
            .all(|(&noisy, &original)| (noisy - original).abs() > 0.3); //threshold chosen for test validation
        assert!(
            changed,
            "Con 0 dB di rumore, ci si aspettava una variazione significativa"
        );
    }

    #[test]
    fn test_execute_low_noise() {
        // Test con basso rumore (40 dB), le variazioni dovrebbero essere minime
        let awgn = AWGN::new(40.0); // Minimo rumore
        let symbols = vec![-1.0, 1.0, -1.0, 1.0, 1.0];
        let noisy_output = awgn.execute(&symbols);

        // Controlliamo che ogni simbolo sia molto vicino al valore originale
        let close_enough = noisy_output
            .iter()
            .zip(symbols.iter())
            .all(|(&noisy, &original)| (noisy - original).abs() < 0.1);
        assert!(
            close_enough,
            "Con 40 dB di rumore, le variazioni dovrebbero essere minime"
        );
    }

    #[test]
    fn test_execute_empty_vector() {
        // Test su un vettore vuoto: dovrebbe restituire un vettore vuoto
        let awgn = AWGN::new(10.0);
        let symbols: Vec<f64> = vec![];
        let noisy_output = awgn.execute(&symbols);
        assert!(noisy_output.is_empty());
    }
}
