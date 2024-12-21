use crate::turbof::turbo_encoder::TurboEncoder;
use crate::turbof::utils::{generate_binary_vector, generate_pn_sequence};
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

        let ndeg = (self.simulation_length).ilog2(); // equivalent to floor(log2(L))

        let ngen=1; //iniizializzazione arbitraria dal codice

        // Generate the PN sequence
        let mut pn_seq = generate_pn_sequence(ndeg as usize, ngen);

        //uso mia funzione per generare input binario casuale
       //let mut pn_seq=generate_binary_vector(self.simulation_length);
       //let mut pn_seq=vec![1,0,0,1,0,1,1,0,1,1];

       println!("pn seq {:?}", pn_seq);
        let ls2 = self.simulation_length - 2usize.pow(ndeg as u32) + 1; // Remainder of the total simulation length that cannot be generated by PNSEQ

        // Initialize the uniform number generator and extend pn_seq to reach simulation length
        let mut utot: Vec<i32> = pn_seq.clone();
        utot.insert(0,i32::MIN);
        for i in 1..=ls2 {
            let rand_val: f64 = rand::random();

            let value = if rand_val <= 0.5 { 1 } else { -1 };
            utot.insert(i+2_usize.pow(ndeg)-1,value);
        }
        utot.remove(0); //rimuovo il valore aggiunto all'inizio minimo
        //da usare se uso la funzione originale del prof

        // Initialize the AWGN generator for the loop
        let mut total_error = 0.0;
        let mut err_res=Vec::new();

        for k1 in 0..m { // va lasciata così?
            // Select the portion of the sequence needed for mptst2br
            let start_idx = k1 * self.block_size;
            let end_idx = (k1 + 1) * self.block_size;
            let u: Vec<i32> = utot[start_idx..end_idx].to_vec();
            let mut err:Vec<Vec<f64>>=vec![vec![0.0; m];self.iterations];
            // Simulate the decoding process
            err_res= self.simulate_process(u, err.clone(), k1);
            total_error += err_res[self.iterations-1][k1];
        }

        // Calculate the average error probability
        let mut toterr = total_error / m as f64; // Dividi il valore di toterr per M
        // let blkstd = blkstd / M;  // Simile per blkstd se necessario

        //println!("Toterr: {:?}", toterr);

        let mut erriter = vec![0.0; self.iterations];  // Inizializza erriter con zeri (o con tipo adeguato come f64)

        for j in 0..self.iterations {
            erriter[j] = err_res[j].iter().sum::<f64>() / m as f64;  // Somma gli errori e divide per M, err_res è un vec di vec !!
        }

        toterr
    }

    // Simulate encoding and decoding process
    fn simulate_process(&self, vec_block: Vec<i32>, err: Vec<Vec<f64>>, k1: usize) -> Vec<Vec<f64>>{
        //dovrei mettere bits to levels qui e non fare nulla dopo, inoltre capire bene utot inizializzato cosa fa
        // Placeholder for the actual decoding process using MPTST2B and error calculation
       // println!("vec_block {:?}", vec_block);
       // let vec_levels=utils::bits_to_levels(vec_block);
        //println!("vec_levels {:?}", vec_levels);
        let mut encoder =TurboEncoder::new(vec_block, self.perm.clone());
        //VEC BLOCK è GIA IN LIVELLI
        let (u,up, sys1, sys2)=encoder.encode();
        //println!("sys2 {:?}", sys2);

        //println!("after encode u {:?} up {:?} sys1 {:?} sys2 {:?}", u.len(), up.len(), sys1.len(), sys2.len());

        let ls = u.len() - 2; // Calcola la lunghezza dei dati originali, senza l'estensione

        let n=generate_noise_vector(ls, self.error_probability); //vettore di rumore in levels

        //println!("noise: {:?}", n);

        //let u_levels=utils::bits_to_levels(u);
        //println!("u levels {:?}", u_levels.len());
        //println!("sys 2 normale {:?}", sys2.len());
        //let sys1_levels=utils::bits_to_levels(sys1);
        //let sys2_levels=utils::bits_to_levels(sys2);

        //println!("sys2 levels {:?}", sys2_levels);
        // Primo bit sistematico con rumore
        let rs1: Vec<i32> = u.iter().zip(&n[0..ls + 2]).map(|(&ui, &ni)| ui * ni).collect();

        // Upper RSC con rumore
        let ry1: Vec<i32> = sys1.iter()
            .zip(&n[ls + 2..2 * ls + 4])
            .map(|(&yi, &ni)| yi * ni)
            .collect();
        //println!("ry1 {:?}", ry1);


        // Lower RSC con rumore
        let ry2: Vec<i32> = sys2.iter()
            .zip(&n[2 * ls + 4..3 * ls + 6])
            .map(|(&yi, &ni)| yi * ni)
            .collect();
       //println!("ry2 {:?}", ry2);
//println!("ry2 {:?}", ry2.len());
  //      println!("ls {:?}", ls);

        //ls adesso vale la lunghezza originale senza estensione
        //up è interleaved, u2 è dopo la perturbazione ovvero rs1
        //provo a usare i livelli

        let mut turbo_decoder =TurboDecoder::new(u.clone(), rs1, ry1, ry2, self.error_probability, 1, ls, up); //r13 vale 1 rate 1/3

        turbo_decoder.decode(self.iterations, self.perm.clone(), err.clone(), k1)

        //ritorna il decoder il vettore di errori
    }
}