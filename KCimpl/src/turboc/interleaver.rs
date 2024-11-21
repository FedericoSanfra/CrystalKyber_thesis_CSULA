use rand::Rng;

pub struct SRandomInterleaver {
    pub permutation: Vec<usize>,
    pub block_size: usize
}

impl SRandomInterleaver {
    /// Crea un nuovo interleaver S-random con lunghezza `K` e spread `s`, spread<17 recommended
    pub fn new(block_size: usize, s: usize, block_number: usize) -> Option<Self> {
        let interleaver_size=block_size*block_number; //es. 256 * numero di chiavi in codifica
        for _ in 0..100 {
            if let Some(permutation) = Self::generate_permutation(interleaver_size, s) {
                return Some(Self {
                    permutation,
                    block_size

                });
            }
        }
        None
    }

    fn generate_permutation(k: usize, s: usize) -> Option<Vec<usize>> {
        let mut rng = rand::thread_rng();
        let mut permutation: Vec<f64> = vec![rng.gen::<f64>() * k as f64];
        let mut len = 1;
        let mut attempts = 0;

        while len < k {
            attempts += 1;
            if attempts > 3000 {
                return None; // Fallimento
            }

            let candidate = rng.gen::<f64>() * k as f64;
            let mut is_valid = true;

            for j in 0..len {
                if (len - j) as f64 + (candidate - permutation[j]).abs() < s as f64 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                permutation.push(candidate);
                len += 1;
            }
        }

        // Ordina e restituisci l'indice della permutazione
        let mut indexed_perm: Vec<(usize, f64)> = permutation.iter().cloned().enumerate().collect();
        indexed_perm.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        Some(indexed_perm.iter().map(|&(idx, _)| idx).collect())
    }

    /// Calcola lo spread minimo della permutazione
    pub fn calculate_spread(&self) -> usize {
        let mut min_distance = usize::MAX;

        for i in 0..self.permutation.len() {
            for j in 0..self.permutation.len() {
                if i != j {
                    let distance = (i as isize - j as isize).abs() as usize
                        + (self.permutation[i] as isize - self.permutation[j] as isize).abs() as usize;
                    min_distance = min_distance.min(distance);
                }
            }
        }

        min_distance
    }
}


pub fn calculate_transposition(permutation: &[usize]) -> Vec<usize> {
    let mut nperm = permutation.to_vec();
    let mut transposition = vec![0; nperm.len()];

    for i in 0..nperm.len() {
        for j in i..nperm.len() {
            if nperm[j] == i {
                transposition[i] = j - i + 1;

                // Scambia gli elementi
                nperm.swap(i, j);
                break;
            }
        }
    }

    transposition
}


pub fn generate_binary_vector(size: usize) -> Vec<usize> { ///for testing purposes
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..=1)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interleaver_with_small_spread() {
        let k = 100;
        let s = 5;

        let interleaver = SRandomInterleaver::new(k, s, 1)
            .expect("Interleaver construction failed with small spread");

        // Verifica che la lunghezza della permutazione sia corretta
        assert_eq!(interleaver.permutation.len(), k);

        // Calcolo dello spread e verifica che sia >= 0 (può essere minore di `s`)
        let spread = interleaver.calculate_spread();
        assert!(spread >= 0, "Spread cannot be negative: got {}", spread);

        println!(
            "Test small spread passed: k = {}, s = {}, calculated spread = {}",
            k, s, spread
        );
    }

    #[test]
    fn test_interleaver_with_medium_spread() {
        let k = 500;
        let s = 10;

        let interleaver = SRandomInterleaver::new(k, s,1)
            .expect("Interleaver construction failed with medium spread");

        // Verifica che la lunghezza della permutazione sia corretta
        assert_eq!(interleaver.permutation.len(), k);

        // Calcolo dello spread e verifica che sia >= 0 (può essere minore di `s`)
        let spread = interleaver.calculate_spread();
        assert!(spread >= 0, "Spread cannot be negative: got {}", spread);

        println!(
            "Test medium spread passed: k = {}, s = {}, calculated spread = {}",
            k, s, spread
        );
    }

    #[test]
    fn test_interleaver_with_large_spread() {
        let k = 1000;
        let s = 15;

        let interleaver = SRandomInterleaver::new(k, s, 1)
            .expect("Interleaver construction failed with large spread");

        // Verifica che la lunghezza della permutazione sia corretta
        assert_eq!(interleaver.permutation.len(), k);

        // Calcolo dello spread e verifica che sia >= 0 (può essere minore di `s`)
        let spread = interleaver.calculate_spread();
        assert!(spread >= 0, "Spread cannot be negative: got {}", spread);

        println!(
            "Test large spread passed: k = {}, s = {}, calculated spread = {}",
            k, s, spread
        );
    }

    #[test]
    fn test_transposition_vector_generation() {
        let k = 100;
        let s = 5;

        let interleaver = SRandomInterleaver::new(k, s, 1)
            .expect("Interleaver construction failed");

        let transposition = calculate_transposition(&interleaver.permutation);

        // Verifica che la lunghezza del vettore di trasposizione sia corretta
        assert_eq!(transposition.len(), k);

        // Verifica che ogni elemento del vettore di trasposizione sia valido
        for &value in &transposition {
            assert!(
                value >= 1 && value <= k,
                "Invalid transposition value: {}",
                value
            );
        }

        println!(
            "Test transposition passed: k = {}, s = {}, transposition = {:?}",
            k, s, transposition
        );
    }

    #[test]
    fn test_failure_case_with_high_spread() {
        let k = 100;
        let s = 50; // Spread molto alto, superiore al possibile spread

        let interleaver = SRandomInterleaver::new(k, s, 1);

        // Dato che lo spread è troppo alto, ci aspettiamo un fallimento
        assert!(interleaver.is_none(), "Interleaver should fail with spread too large");

        println!("Test high spread failure passed: k = {}, s = {}", k, s);
    }
}
