use crate::turbof::interleaver::Interleaver;
use crate::turbof::siso_decoder::SISO;
use crate::turbof::utils::bits_to_symbols;

pub struct TurboDecoder {
    siso: SISO,
    interleaver: Interleaver,
    iterations: usize,
}

impl TurboDecoder {

    pub fn new(siso: SISO, interleaver: Interleaver, iterations: usize)->Self{
        Self {
            siso,
            interleaver,
            iterations
        }
    }
    pub fn decode(
        &self,
        systematic: &[f64],
        parity1: &[f64],
        parity2: &[f64],
        u: &[i32], // Aggiunto per calcolo dell'errore
    ) -> Vec<f64> {
        let mut extrinsic = vec![0.0; systematic.len()];
        let mut decoded = vec![0.0; systematic.len()];

        for _ in 0..self.iterations {
            let (app1, dec1, _) = self.siso.decode(systematic, parity1, parity2, &extrinsic, u);

            let extrinsic1: Vec<f64> = app1
                .iter()
                .zip(systematic.iter())
                .map(|(&app, &sys)| app[0] - sys) // Usando app[0] come esempio
                .collect();

            let interleaved_extrinsic = self.interleaver.interleave(extrinsic1.clone());

            let interleaved_extrinsic_converted=bits_to_symbols(&interleaved_extrinsic);

            let (app2, dec2, _) = self.siso.decode(&interleaved_extrinsic_converted, parity2, parity1, &extrinsic, u);

            let diff = app2
                .iter()
                .zip(interleaved_extrinsic.iter())
                .map(|(&app, &int)| app[0] - int as f64) // Calcola la differenza tra i valori
                .collect::<Vec<f64>>();

            // let deinterleaved_diff2=self.interleaver.deinterleave(&diff);

            // Calcolo della differenza tra app2 e interleaved_extrinsic
            // let diff = app2
            //     .iter()
            //     .zip(interleaved_extrinsic.iter())
            //     .map(|(&app, &int)| app - int) // Calcola la differenza tra i valori
            //     .collect::<Vec<f64>>();

            // Deinterleaving di diff
            let deinterleaved_diff = self.interleaver.deinterleave(&diff);

            // Aggiorniamo extrinsic con il risultato deinterleaved
            extrinsic = bits_to_symbols(&deinterleaved_diff);

            decoded = extrinsic
                .iter()
                .map(|&value| if value >= 0.0 { 1.0 } else { -1.0 })
                .collect();
        }

        decoded
    }
}

