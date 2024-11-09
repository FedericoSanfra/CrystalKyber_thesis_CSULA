use std::f64;
use crate::turboc::trellis::Trellis;

pub struct SISODecoder {
    block_size: usize,
    trellis: Trellis,
    branch_metrics: Vec<Vec<Vec<f64>>>,
    forward_metrics: Vec<Vec<f64>>,
    backward_metrics: Vec<Vec<f64>>,
    llr: Vec<f64>,
}

impl SISODecoder {

    fn expand_states(state: &(usize, usize)) -> Vec<usize> {
        vec![state.0, state.1]
    }
    //passo da vettore di tuple a vettore collection iterabile, per past e future states


    pub fn new(block_size: usize) -> Self {
        let trellis = Trellis::new();
        let branch_metrics = SISODecoder::init_branch_metrics(4, 4, block_size);
        let forward_metrics = SISODecoder::init_path_metric(4, block_size + 1);
        let backward_metrics = SISODecoder::init_path_metric(4, block_size + 1);
        let llr = vec![0.0; block_size];

        SISODecoder {
            block_size,
            trellis,
            branch_metrics,
            forward_metrics,
            backward_metrics,
            llr,
        }
    }

    fn init_branch_metrics(m: usize, n: usize, depth: usize) -> Vec<Vec<Vec<f64>>> {
        vec![vec![vec![0.0; n]; m]; depth]
    }

    fn init_path_metric(m: usize, depth: usize) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![-f64::INFINITY; depth]; m];
        for i in 0..m {
            matrix[i][0] = 0.0;
        }
        matrix
    }

    //creiamo vettore di triple con valori x y demodulati, valori di simboli con errore + la probabilità per la metrica
    fn demultiplex(vector: Vec<f64>) -> Vec<(f64, f64, f64)> {
        vector.chunks(3).map(|chunk| (chunk[0], chunk[1], 0.0)).collect()
    }

    pub fn reset(&mut self) {
        self.branch_metrics = SISODecoder::init_branch_metrics(4, 4, self.block_size);
        self.forward_metrics = SISODecoder::init_path_metric(4, self.block_size + 1);
        self.backward_metrics = SISODecoder::init_path_metric(4, self.block_size + 1);
        self.llr = vec![0.0; self.block_size];
    }

    pub fn compute_branch(&mut self, tuples: &[(f64, f64, f64)]) {
        for k in 0..self.block_size {
            for &(m, n) in &self.trellis.possible_transitions {
                if let Some((i, o)) = self.trellis.transition_to_symbols(m, n) {
                    self.branch_metrics[k][m][n] = i as f64 * tuples[k].0 + o as f64 * tuples[k].1 + i as f64 * tuples[k].2;
                }
            }
        }
    }

    pub fn compute_forward(&mut self, k: usize, state: usize) {
        let past_states = SISODecoder::expand_states(&self.trellis.past_states[state]);


        let forward_metrics: Vec<f64> = past_states.iter().map(|&s| self.forward_metrics[k - 1][s]).collect();
        let branch_metrics: Vec<f64> = past_states.iter().map(|&s| self.branch_metrics[k - 1][s][state]).collect();

        self.forward_metrics[k][state] = Trellis::butterfly(&forward_metrics, &branch_metrics);
    }

    pub fn compute_backward(&mut self, k: usize, state: usize) {
        let future_states = SISODecoder::expand_states(&self.trellis.future_states[state]);

        let r = self.block_size - k;

        let backward_metrics: Vec<f64> = future_states.iter().map(|&s| self.backward_metrics[k - 1][s]).collect();
        let branch_metrics: Vec<f64> = future_states.iter().map(|&s| self.branch_metrics[r][state][s]).collect();

        self.backward_metrics[k][state] = Trellis::butterfly(&backward_metrics, &branch_metrics);
    }

    pub fn compute_llr(&mut self, k: usize) {
        let r = self.block_size - k - 1;

        let mut positive = vec![];
        let mut negative = vec![];

        for &(m, n) in &self.trellis.possible_transitions {
            if let Some((i, _)) = self.trellis.transition_to_symbols(m, n) {
                let forward_metric = self.forward_metrics[k][m];
                let branch_metric = self.branch_metrics[k][m][n];
                let backward_metric = self.backward_metrics[r][n];

                if i < 0 {
                    negative.push(forward_metric + branch_metric + backward_metric);
                } else {
                    positive.push(forward_metric + branch_metric + backward_metric);
                }
            }
        }

        self.llr[k] = positive.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - negative.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    }

    pub fn execute(&mut self, tuples: Vec<(f64, f64, f64)>) -> Vec<f64> {
        self.compute_branch(&tuples);

        for k in 1..=self.block_size {
            for state in 0..4 {
                self.compute_forward(k, state);
                self.compute_backward(k, state);
            }
        }

        for k in 0..self.block_size {
            self.compute_llr(k);
        }

        self.llr.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_siso_decoder_creation() {
        let decoder = SISODecoder::new(10);
        assert_eq!(decoder.block_size, 10);
        assert_eq!(decoder.llr.len(), 10);
        assert!(decoder.branch_metrics.len() == 10);
        assert!(decoder.forward_metrics.len() == 4);
        assert!(decoder.backward_metrics.len() == 4);
    }

    #[test]
    fn test_init_branch_metrics() {
        let branch_metrics = SISODecoder::init_branch_metrics(4, 4, 10);
        assert_eq!(branch_metrics.len(), 10); // Depth check
        assert_eq!(branch_metrics[0].len(), 4); // Number of states check
        assert_eq!(branch_metrics[0][0].len(), 4); // Number of transitions check
    }

    #[test]
    fn test_init_path_metric() {
        let path_metrics = SISODecoder::init_path_metric(4, 10);
        assert_eq!(path_metrics.len(), 4); // Number of states
        assert_eq!(path_metrics[0].len(), 10); // Depth of path metric
        assert_eq!(path_metrics[0][0], 0.0); // Initial state set to 0
        assert_eq!(path_metrics[0][1], -f64::INFINITY); // Other values should be -infinity
    }

    #[test]
    fn test_demultiplex() {
        let vector = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let demuxed = SISODecoder::demultiplex(vector);
        assert_eq!(demuxed.len(), 2);
        assert_eq!(demuxed[0], (1.0, 2.0, 0.0));
        assert_eq!(demuxed[1], (4.0, 5.0, 0.0));
    }

    #[test]
    fn test_reset() {
        let mut decoder = SISODecoder::new(10);
        decoder.reset();

        assert!(decoder.branch_metrics.iter().all(|depth| depth.iter().all(|state| state.iter().all(|&metric| metric == 0.0))));
        assert!(decoder.forward_metrics.iter().all(|state| state[0] == 0.0 && state[1..].iter().all(|&metric| metric == -f64::INFINITY)));
        assert!(decoder.backward_metrics.iter().all(|state| state[0] == 0.0 && state[1..].iter().all(|&metric| metric == -f64::INFINITY)));
        assert!(decoder.llr.iter().all(|&value| value == 0.0));
    }

    // Test for the expansion of state tuples
    #[test]
    fn test_expand_states() {
        let decoder = SISODecoder::new(10);

        // Testing the expansion of different state tuples
        let state_tuple = (1, 2);
        let expanded = SISODecoder::expand_states(&state_tuple);

        assert_eq!(expanded, vec![1, 2]);  // The expected result is a Vec with elements 1 and 2

        let state_tuple = (3, 4);
        let expanded = SISODecoder::expand_states(&state_tuple);

        assert_eq!(expanded, vec![3, 4]);  // The expected result is a Vec with elements 3 and 4
    }
}

