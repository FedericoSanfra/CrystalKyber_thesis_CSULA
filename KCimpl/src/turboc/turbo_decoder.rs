use crate::turboc::siso_decoder::SISODecoder;
use std::cmp::Ordering;

pub struct TurboDecoder {
    interleaver: Vec<usize>,
    block_size: usize,
    tail_bits: usize,
    max_iter: usize,
    decoders: Vec<SISODecoder>,
    llr_ext: Vec<f64>,
}

impl TurboDecoder {
    pub fn new(interleaver: Vec<usize>, tail_bits: usize, max_iter: usize) -> Self {
        let block_size = interleaver.len();
        let decoders = vec![SISODecoder::new(block_size + tail_bits), SISODecoder::new(block_size + tail_bits)];
        let llr_ext = vec![0.0; block_size + tail_bits];

        TurboDecoder {
            interleaver,
            block_size,
            tail_bits,
            max_iter,
            decoders,
            llr_ext,
        }
    }

    fn demultiplex(a: &[f64], b: &[f64], extrinsic: &[f64]) -> Vec<(f64, f64, f64)> {
        a.iter().zip(b.iter()).zip(extrinsic.iter())
            .map(|((&a, &b), &ext)| (a, b, ext))
            .collect()
    }

    fn early_exit(llr: &[f64], llr_ext: &[f64]) -> bool {
        llr.iter().zip(llr_ext.iter())
            .all(|(&l, &e)| l.partial_cmp(&0.0) == Some(Ordering::Greater) && e.partial_cmp(&0.0) == Some(Ordering::Greater))
    }

    fn interleave(&self, vector: &[f64]) -> Vec<f64> {
        let mut interleaved = vec![0.0; vector.len()];
        for (i, &index) in self.interleaver.iter().enumerate() {
            interleaved[i] = vector[index];
        }
        interleaved
    }

    fn deinterleave(&self, vector: &[f64]) -> Vec<f64> {
        let mut deinterleaved = vec![0.0; vector.len()];
        for (i, &index) in self.interleaver.iter().enumerate() {
            deinterleaved[index] = vector[i];
        }
        deinterleaved
    }

    fn iterate(&mut self, vector: &[f64]) -> bool {
        let input_tuples = Self::demultiplex(&vector[0..], &vector[1..], &self.llr_ext);

        let mut llr_1 = self.decoders[0].execute(input_tuples.clone());
        for (i, llr) in llr_1.iter_mut().enumerate() {
            *llr -= self.llr_ext[i] + 2.0 * vector[i];
        }

        let llr_interleaved = self.interleave(&llr_1);
        let input_interleaved: Vec<f64> = self.interleave(&vector.iter().step_by(3).cloned().collect::<Vec<f64>>());

        let input_tuples = Self::demultiplex(&input_interleaved, &vector[2..], &llr_interleaved);

        let mut llr_2 = self.decoders[1].execute(input_tuples.clone());
        for (i, llr) in llr_2.iter_mut().enumerate() {
            *llr -= llr_interleaved[i] + 2.0 * input_interleaved[i];
        }

        self.llr_ext = self.deinterleave(&llr_2);

        Self::early_exit(&llr_1, &self.llr_ext)
    }

    pub fn execute(&mut self, vector: Vec<f64>) -> Vec<f64> {
        for _ in 0..self.max_iter {
            if self.iterate(&vector) {
                break;
            }
        }
        self.llr_ext.clone()
    }
}
