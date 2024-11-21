//livelli in input in output livelli con tasso di errore p, frazione di errore p parametrizzato,
// flippati, in posizione randomica, scorrelati, estrai una variabile tra 0 e 1 per ogni simbolo

//al decodoficatore passo questo vettore
//al posto della varianza c'è p,

//formula llr,
//con p=0 valore max di f64

use rand::Rng;

///channel implementation of BSC with error probability p


pub struct BSC {
    error_prob: f64, // Probabilità di errore p, compresa tra 0 e 1
    pub sigma: f64
}

impl BSC {
    // Crea una nuova istanza di BSC con una probabilità di errore specifica tra 0 e 1
    pub fn new(error_prob: f64) -> Self {
       if error_prob<=1.0 && error_prob>= 0.0 {
           Self{
               error_prob,
               sigma: ((1.0 - error_prob) / error_prob).ln() //livello -1.0 oppure 0
           }
       } else {
           Self {
               error_prob: 0.0, //se considerato non valida, metto di default 0.0, nessun errore
               sigma: f64::INFINITY
           }
       }
    }

    // Metodo per convertire un vettore di bit (0,1) in simboli (+1, -1)
    pub fn convert_to_symbols(bits: &[u8]) -> Vec<f64> {
        bits.iter().map(|&bit| if bit == 0 { -1.0 } else { 1.0 }).collect()
    } //per ora tengo i f64 per comodità

    // Metodo per applicare il flipping ai simboli in base alla probabilità di errore p
    pub fn execute(&self, symbols: &[f64]) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        symbols
            .iter()
            .map(|&symbol| {
                // Se generiamo un numero casuale inferiore a `error_prob`, flippiamo il simbolo
                if rng.gen::<f64>() < self.error_prob { //generating a (0..1) number
                    symbol * -1.0 // Flipping
                } else {
                    symbol
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_symbols() {
        let input_bits = vec![1, 0, 0, 1, 0, 1, 1 , 0];
        let expected_symbols = vec![1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0];
        let symbols = BSC::convert_to_symbols(&input_bits);
        assert_eq!(symbols, expected_symbols, "Conversione dei bit in simboli fallita");
    }

    #[test]
    fn test_execute_no_error() {
        let bsc = BSC::new(0.0); // Nessun errore
        let symbols = vec![1.0, -1.0, -1.0, 1.0];
        let output_symbols = bsc.execute(&symbols);
        assert_eq!(symbols, output_symbols, "Con p=0, nessun simbolo dovrebbe essere flippato");
    }

    #[test]
    fn test_execute_full_error() {
        let bsc = BSC::new(1.0); // Sempre errore
        let symbols = vec![1.0, -1.0, -1.0, 1.0];
        let expected_flipped = vec![-1.0, 1.0, 1.0, -1.0];
        let output_symbols = bsc.execute(&symbols);
        assert_eq!(output_symbols, expected_flipped, "Con p=1, tutti i simboli devono essere flippati");
    }

    #[test]
    fn test_execute_random_error() {
        let bsc = BSC::new(0.5); // Probabilità di errore al 50%
        let symbols = vec![1.0, -1.0, -1.0, 1.0];
        let output_symbols = bsc.execute(&symbols);
        assert_eq!(symbols.len(), output_symbols.len(), "La lunghezza dell'output deve essere uguale all'input");
        // Test qualitativo: verificare che alcuni simboli siano flippati, ma non tutti
        let flipped_count = symbols
            .iter()
            .zip(output_symbols.iter())
            .filter(|(&original, &flipped)| original != flipped)
            .count();
        println!(" output symbols: {:?}", output_symbols);
        println!(" input symbols: {:?}", symbols);
        assert!(flipped_count > 0, "Alcuni simboli devono essere flippati con p=0.5");
        assert!(flipped_count < symbols.len(), "Non tutti i simboli devono essere flippati con p=0.5");
    }
}

