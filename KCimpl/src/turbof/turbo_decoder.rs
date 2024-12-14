use crate::turbof::siso_decoder::SISODecoder;
use crate::turbof::interleaver::Interleaver;
pub struct TurboDecoder {
    siso1: SISODecoder,
    siso2: SISODecoder,
    u: Vec<i32>,
    up: Vec<i32>,
    u2: Vec<i32>,
    y1: Vec<f64>,
    y2: Vec<f64>,
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

    pub fn new(u: Vec<i32>, p1: f64, r13: i32, ls: usize, up: Vec<i32>) -> Self {
        Self {
            siso1: SISODecoder::new(),
            siso2: SISODecoder::new(),
            u,
            up,
            u2: vec![0; ls + 2],
            y1: vec![0.0; ls + 2],
            y2: vec![0.0; ls + 2],
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

    pub fn decode(&mut self, niter: usize, perm: Vec<i32>, err: Vec<Vec<f64>>, k1: usize) -> Vec<i32> {

        // Calculating statistics for each received bit stream
        for i in 0..self.ls + 2 {
            self.gam_sys1[i] = 0.0;
            self.gam_sys2[i] = 0.0;

            // For rate 1/3, systematic bit is transmitted
            if self.r13 == 1 {
                if self.u2[i] == 1 {
                    self.gam_sys2[i] -= (1.0 - self.p1).ln() - self.p1.ln();
                } else if self.u2[i] == -1 {
                    self.gam_sys2[i] += (1.0 - self.p1).ln() - self.p1.ln();
                }
            }

            // For upper RSC (y1), using the received values
            self.gam_ry11[i] = 0.0;
            self.gam_ry12[i] = -self.y1[i] * ((1.0 - self.p1).ln() - self.p1.ln());

            // For lower RSC (y2), using the received values
            self.gam_ry21[i] = 0.0;
            self.gam_ry22[i] = -self.y2[i] * ((1.0 - self.p1).ln() - self.p1.ln());
        }

        self.gam_syso2.clone_from(&self.gam_sys2);

        ///DECODING LOOP MAP ALGORITHM  A POSTERIORI
        for iteration in 0..niter {
            let perm2 = perm.clone();  // Clone di perm per usarlo in questa iterazione

            // Chiamata alla funzione sisoRSCmem2, che restituisce (app1, dec1, count1)
            let (app1, _dec1, count1) = self.siso1.decode(self.ls, self.u.clone(), &self.gam_ry11, &self.gam_ry12, &self.gam_sys1, &self.gam_sys2);

            // Calcolo della differenza tra la seconda e la prima riga di app1
            let mut dapp1 = vec![0.0; self.ls];
            for i in 0..self.ls {
                dapp1[i] = app1[1][i] - app1[0][i];
            }

            // Calcolo dell'errore per questa iterazione
            err[iteration][k1] = count1 as f64/ self.ls as f64;

            //PREPARATION FOR NEXT SECTION 2 SISO DECODER
            // Inizializzazione di gamsys2 e ern
            self.gam_sys2=Vec::new();  // gamsys2 inizializzato come vettore vuoto
            let mut ern = dapp1.clone();  // ern è il risultato della differenza tra le righe di app1

            // Chiamata alla funzione mapint
            let out = Interleaver::mapint(self.ls, ern, perm2);

            // Costruzione di gamsys2 con out e aggiungendo 0, 0
            self.gam_sys2.extend_from_slice(&out);  // Aggiunge i valori di out in gamsys2
            self.gam_sys2.push(0.0);  // Aggiunge 0
            self.gam_sys2.push(0.0);  // Aggiunge 0

            // Pulizia di out e ern
            let out: Vec<f64> = Vec::new();  // Resetta out
            let ern: Vec<f64> = Vec::new();  // Resetta ern
            ///LOWER RSC SISODECODER
            ///
            // Chiamata alla funzione `sisoRSCmem2` che ritorna app2, dec2 e count2
            let (app2, dec2, count2) = self.siso2.decode(self.ls, self.up.clone(), &self.gam_ry21, &self.gam_ry22, &self.gam_sys1, &self.gam_sys2);

            // Calcolo della differenza tra le due righe di app2
            let dapp2: Vec<f64> = app2[1..self.ls].iter().zip(app2[0..self.ls].iter()).map(|(&x1, &x0)| x1[1] - x0[0]).collect();
            //app2 è un Vec<Vec<f64>>






        }

        unimplemented!()


    }
}