use crate::turbof::siso_decoder::SISODecoder;
use crate::turbof::mapints;
use crate::turbof::utils::{apply_permutation, reverse_permutation, transpositions_to_permutations};

pub struct TurboDecoder {
    siso1: SISODecoder,
    siso2: SISODecoder,
    u: Vec<i32>,
    up: Vec<i32>,
    u2: Vec<i32>,
    ry1: Vec<i32>,
    ry2: Vec<i32>,
    gam_sys1: Vec<f64>,
    gam_sys2: Vec<f64>,
    gam_syso2: Vec<f64>,
    gam_ry11: Vec<f64>,
    gam_ry12: Vec<f64>,
    gam_ry21: Vec<f64>,
    gam_ry22: Vec<f64>,
    p1: f64,  // Probabilità di errore
    r13: i32, // Tipo di codifica (1/2 o 1/3)
    ls: usize, // Lunghezza del messaggio
}

impl TurboDecoder{

    pub fn new(u: Vec<i32>, u2: Vec<i32>, ry1: Vec<i32>, ry2: Vec<i32>, p1: f64, r13: i32, ls: usize, up: Vec<i32>) -> Self {
        Self {
            siso1: SISODecoder::new(),
            siso2: SISODecoder::new(),
            u,
            up,
            u2,
            ry1,
            ry2,
            gam_sys1: vec![0.0; ls + 2],
            gam_sys2: vec![0.0; ls + 2],
            gam_syso2: vec![0.0; ls + 2],
            gam_ry11: vec![0.0; ls + 2],
            gam_ry12: vec![0.0; ls + 2],
            gam_ry21: vec![0.0; ls + 2],
            gam_ry22: vec![0.0; ls + 2],
            p1,
            r13,
            ls,
        }
    }

    pub fn decode(&mut self, niter: usize, perm: Vec<i32>, err: Vec<Vec<f64>>, k1: usize) -> Vec<Vec<f64>>{
        let mut err=err.clone();
        // Calculating statistics for each received bit stream
        //chiarire meglio operazione
        for j in 0..self.ls+2{
            let value:f64=rand::random();
            if value<=self.p1{
                self.u2[j]=-1*self.u[j]
            } else{
                self.u2[j]=self.u[j]
            }
        }

        for i in 0..self.ls + 2 {
            self.gam_sys1[i] = 0.0;
            self.gam_sys2[i] = 0.0;

            // For rate 1/3, systematic bit is transmitted
            if self.r13 == 1 {
                if self.u2[i] == 1 {
                    self.gam_sys2[i] -= ((1.0-self.p1)/(self.p1)).ln();
                } else if self.u2[i] == -1 {
                    self.gam_sys2[i] += ((1.0-self.p1)/(self.p1)).ln();
                }
            }
            //println!("gam_sys2"); //istogramma di ry12 prima e dopo il siso

            // For upper RSC (y1), using the received values
            self.gam_ry11[i] = 0.0;
            self.gam_ry12[i] = -self.ry1[i] as f64 * ((1.0-self.p1)/(self.p1)).ln();

            // For lower RSC (y2), using the received values
            self.gam_ry21[i] = 0.0;
            self.gam_ry22[i] = -self.ry2[i] as f64 * ((1.0-self.p1)/(self.p1)).ln();
        }

        //println!("gamsys prima decode {:?}", &self.gam_sys2);

        self.gam_syso2=self.gam_sys2.clone();

        ///DECODING LOOP MAP ALGORITHM  A POSTERIORI
        for iteration in 0..niter {
            let perm2 = perm.clone();  // Clone di perm per usarlo in questa iterazione

            //println!("gamsys2 fuori decode {:?}", &self.gam_sys2);
            // Chiamata alla funzione sisoRSCmem2, che restituisce (app1, dec1, count1)

            //println!("gamry12 PRIMA SISO: {:?}", self.gam_ry12);
            let (app1, _dec1, count1) = self.siso1.decode(self.ls, self.u.clone(), &self.gam_ry11, &self.gam_ry12, &self.gam_sys1, &self.gam_sys2);
            //println!("app1 DOPO : {:?}", &app1);
            // Calcolo della differenza tra la seconda e la prima riga di app1
            let mut dapp1 = vec![0.0; self.ls];
            for i in 0..self.ls {
                dapp1[i] = app1[1][i] - app1[0][i];
            }
            // Calcolare la somma dei valori assoluti
            let sum: f64 = dapp1.iter().map(|&x| x.abs()).sum();

            // Calcolare la media
            let average = sum / dapp1.len() as f64;
            println!("average iterazione {:?} media {:?}", iteration, average);

           // println!("dapp1 dopo siso {:?}", dapp1);
           // println!("dapp1 {:?}", dapp1);
            // Calcolo dell'errore per questa iterazione
            err[iteration][k1] = count1 as f64/ self.ls as f64;
            println!("count1 {:?}", count1);

            //PREPARATION FOR NEXT SECTION 2 SISO DECODER
            // Inizializzazione di gamsys2 e ern
            self.gam_sys2=vec![0.0;self.ls +2]; // gamsys2 inizializzato come vettore vuoto
            let mut ern = dapp1.clone();  // ern è il risultato della differenza tra le righe di app1

            // Chiamata alla funzione mapint
            let mut out = mapints::mapint_f64(self.ls, ern.clone(), perm2.clone());
            //let perm_new=transpositions_to_permutations(perm.clone());
            //let out=apply_permutation(ern.clone(),perm_new);
            // Costruzione di gamsys2 con out e aggiungendo 0, 0
            out.push(0.0);
            out.push(0.0);
            self.gam_sys2=out; // Aggiunge i valori di out in gamsys2


            // Pulizia di out e ern
            let out: Vec<f64> = vec![0.0;self.ls+2];  // Resetta out
            let ern: Vec<f64> = vec![0.0; self.ls];  // Resetta ern
            ///LOWER RSC SISODECODER
            ///
            // Chiamata alla funzione `sisoRSCmem2` che ritorna app2, dec2 e count2

            let (app2, dec2, count2) = self.siso2.decode(self.ls, self.up.clone(), &self.gam_ry21, &self.gam_ry22, &self.gam_sys1, &self.gam_sys2);

            //println!(" app2 0 {:?}", &app2[0][0..50]);
            //println!(" app2 1 {:?}", &app2[1][0..50]);
            let mut dapp2 = vec![0.0; self.ls];
            for i in 0..self.ls {
                dapp2[i] = app2[1][i] - app2[0][i];
            }
            //println!("dapp2 dopo {:?}", dapp2[0]);
            // Calcolo della differenza tra le due righe di app2
            // let dapp2: Vec<f64> = app2[1..self.ls]
            //     .iter()
            //     .zip(app2[0..self.ls].iter())
            //     .map(|(x1, x0)| x1[1] - x0[0]) // Usa riferimenti per accedere ai valori
            //     .collect();

            //app2 è un Vec<Vec<f64>>
            err[iteration][k1] = count2 as f64/ self.ls as f64;
            //println!("count2 {:?}", count2);
            //sostituisco i valori
            // Generating the extrinsic information:
            // This is done by treating the output metrics as being composed
            // of two parts. One part is due to an effective systematic bit whose
            // soft metric is actually the permuted dapp (i.e., gamsys2),
            // and a second part which is the extrinsic information generated by the lower RSC.

            // Estimating the extrinsic information by subtracting gamsys2 from dapp2
            let mut ext1: Vec<f64> = dapp2.iter()
                .zip(self.gam_sys2.iter())
                .map(|(&x, &y)| x - y)
                .collect();
            //println!("ext 1 -> {:?}", ext1);

            // Estimating the variance of the extrinsic information:
            let variance: f64 = ext1.iter()
                .map(|&x| (x - ext1.iter().sum::<f64>() / ext1.len() as f64).powi(2))
                .sum::<f64>() / ext1.len() as f64;

            // Standard deviation
            let std_deviation = variance.sqrt();
            println!("std deviation {:?} iterazione {:?} blocco k1 {:?}",std_deviation, iteration, k1);

            // Store the result for testing (not needed, used for testing)
           // std[iteration][k1] = std_deviation;

            // Reset gamsys2 for next use
            self.gam_sys2=vec![0.0; self.ls+2];

            // SECOND GLOBAL DECODING ITERATION:

            // Now we need the inverse of the permutation used above.
            // The transposition vector of the FSP associated with this permutation is loaded:
            let invp = mapints::invperm(perm.clone());  // La funzione invperm è già implementata separatamente

            // perm2 is set to the inverse permutation
            let mut perm2 = invp.clone();

            // Interface with the interleaving routine
            // The last four elements of the invp are [3, 3, 2, 1].
            // This will be used to finish up the inverse permutation.
            // This information is embedded in mapdint1.m script that performs the deinterleaving operation.

            // ern is set to ext1
            let mut ern = ext1.clone();  // Copia di ext1



            // Calling the deinterleaving routine:
            let mut out = mapints::mapdint_f64(self.ls, ern, perm2);  // La funzione mapdint è già implementata e funziona correttamente
            //let new_perm=transpositions_to_permutations(perm.clone());
            //let mut out=reverse_permutation(ern.clone(), new_perm);
            // permuted ext1 is stored in pext1
            let pext1 = out.clone();  // Permuted ext1

            out.push(0.0);
            out.push(0.0);//estensione di out

            // Extrinsic information is added to the soft metric for the systematic bit.
            self.gam_sys2 = self.gam_syso2.iter().zip(out.iter()).map(|(x, y)| x + *y).collect::<Vec<_>>();
            // sommo i singoli elementi
            // Cleaning up
            ern=Vec::new();
            out=Vec::new();


        }
        err

    }
}