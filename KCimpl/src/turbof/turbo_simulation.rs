use crate::turbof::turbo_encoder::TurboEncoder;
use crate::turbof::utils::generate_pn_sequence;
use crate::turbof::bsc_channel::generate_noise_vector;
use crate::turbof::turbo_decoder::TurboDecoder;
use crate::turbof::utils;

pub struct TurboSimulation {
    perm: Vec<i32>,
    block_size: usize,
    simulation_length: usize,
    error_probability: f64,
    iterations: usize,
    rate: usize //1 per 1/3 e 0 per 1/2
}

impl TurboSimulation{

    pub fn new(
        perm: Vec<i32>,
        block_size: usize,
        simulation_length: usize,
        error_probability: f64,
        iterations: usize,
        rate: usize
    )->Self{
        Self{
            perm,
            block_size,
            simulation_length,
            error_probability,
            iterations,
            rate
        }
    }

    pub fn run_simulation(&self,)->f64{

        // Assume that L and blok are defined earlier
        let m = self.simulation_length / self.block_size; // Number of times mptst2b gets called

        let ls = self.block_size; // this is passed to mptst2b

        let ndeg = (self.simulation_length).log2().floor(); // equivalent to floor(log2(L))

        let ngen=1; //iniizializzazione arbitraria dal codice

        let pn_seq=generate_pn_sequence(ndeg, ngen);


        // Generate the PN sequence
        let mut pn_seq = generate_pn_sequence(ndeg as usize, ngen);

        let ls2 = self.simulation_length - 2usize.pow(ndeg as u32) + 1; // Remainder of the total simulation length that cannot be generated by PNSEQ

        // Initialize the uniform number generator and extend pn_seq to reach simulation length
        let mut utot: Vec<i32> = pn_seq.clone();
        for i in 0..ls2 {
            let rand_val: f64 = rand::random();
            let value = if rand_val <= 0.5 { 1 } else { -1 };
            utot.push(value);
        }

        // Initialize the AWGN generator for the loop
        let mut total_error = 0.0;

        for k1 in 0..m {
            // Select the portion of the sequence needed for mptst2br
            let start_idx = k1 * self.block_size;
            let end_idx = (k1 + 1) * self.block_size;
            let u: Vec<i32> = utot[start_idx..end_idx].to_vec();
            let mut err:Vec<Vec<f64>>=vec![vec![0.0; m];self.iterations];
            // Simulate the decoding process
            total_error += self.simulate_process(u, err.clone(), k1);
        }

        // Calculate the average error probability
        total_error / m as f64
    }

    // Simulate encoding and decoding process
    fn simulate_process(&self, vec_block: Vec<i32>, err: Vec<Vec<f64>>, k1: usize) -> f64 {
        // Placeholder for the actual decoding process using MPTST2B and error calculation
        let mut encoder =TurboEncoder::new(vec_block, self.perm.clone());
        let (u,up, sys1, sys2)=encoder.encode();

        let ls = u.len() - 2; // Calcola la lunghezza dei dati originali, senza l'estensione

        let n=generate_noise_vector(ls, self.error_probability); //vettore di rumore in levels

        let u_levels=utils::bits_to_levels(u);
        let sys1_levels=utils::bits_to_levels(sys1);
        let sys2_levels=utils::bits_to_levels(sys2);

        // Primo bit sistematico con rumore
        let rs1: Vec<i32> = u_levels.iter().zip(&n[0..ls + 2]).map(|(&ui, &ni)| ui * ni).collect();

        // Upper RSC con rumore
        let ry1: Vec<i32> = sys1_levels.iter()
            .zip(&n[ls + 2..2 * ls + 4])
            .map(|(&yi, &ni)| yi * ni)
            .collect();

        // Lower RSC con rumore
        let ry2: Vec<i32> = sys2_levels.iter()
            .zip(&n[2 * ls + 4..3 * ls + 6])
            .map(|(&yi, &ni)| yi * ni)
            .collect();


        //ls adesso vale la lunghezza originale senza estensione
        //provo a usare i livelli

        let mut turbo_decoder =TurboDecoder::new(u_levels.clone(), self.error_probability, 1, ls, up); //r13 vale 1 rate 1/3

        turbo_decoder.decode(self.iterations, self.perm.clone(), err, k1);
        // For now, we will just return a random error rate for illustration
        let rand_error: f64 = rand::random();
        rand_error
    }
}