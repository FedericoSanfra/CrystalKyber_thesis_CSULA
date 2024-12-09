pub struct SISO;

impl SISO {
    /// Decodifica un blocco con il metodo SISO
    pub fn decode(
        &self,
        systematic: &[f64],
        parity1: &[f64],
        parity2: &[f64],
        extrinsic: &[f64],
        u: &[i32],
    ) -> (Vec<[f64; 2]>, Vec<i32>, usize) {
        let len = systematic.len();
        let mut alf = vec![[0.0; 4]; len + 2];
        let mut bet = vec![[0.0; 4]; len + 1];
        let mut app = vec![[0.0; 2]; len];
        let mut dec = vec![0; len];

        // Calcolo della ricorsione Alpha
        self.alpha_recursion(len, parity1, parity2, systematic, extrinsic, &mut alf);

        // Calcolo della ricorsione Beta
        self.beta_recursion(len, parity1, parity2, systematic, extrinsic, &mut bet);

        // Generazione delle metriche APP e decoding
        self.app_metric_generation(len, parity1, parity2, systematic, extrinsic, &alf, &bet, &mut app, &mut dec);

        // Calcolo del conteggio degli errori
        let error_count = dec.iter().zip(u).filter(|(&d, &u)| d != u).count();


        (app, dec, error_count)
    }

    /// Ricorsione Alpha
    fn alpha_recursion(
        &self,
        len: usize,
        parity1: &[f64],
        parity2: &[f64],
        systematic: &[f64],
        extrinsic: &[f64],
        alf: &mut Vec<[f64; 4]>,
    ) {
        let mut ao = [0.0, -100.0, -100.0, -100.0];

        for i in 1..=len {
            let gp = [parity1[i - 1], parity2[i - 1]];
            let gsys = [systematic[i - 1], extrinsic[i - 1]];
            let mut an = [0.0; 4];

            for j in 0..4 {
                an[j] = self.alpha1(&ao, &gp, &gsys, j);
            }

            // Normalizzazione
            let offset = an[0];
            an.iter_mut().for_each(|v| *v -= offset);

            alf[i] = an;
            ao = an;
        }
    }

    /// Ricorsione Beta
    fn beta_recursion(
        &self,
        len: usize,
        parity1: &[f64],
        parity2: &[f64],
        systematic: &[f64],
        extrinsic: &[f64],
        bet: &mut Vec<[f64; 4]>,
    ) {
        let mut bo = [0.0, -100.0, -100.0, -100.0];

        for i in (1..=len).rev() {
            let gp = [parity1[i - 1], parity2[i - 1]];
            let gsys = [systematic[i - 1], extrinsic[i - 1]];
            let mut bn = [0.0; 4];

            for j in 0..4 {
                bn[j] = self.beta1(&bo, &gp, &gsys, j);
            }

            // Normalizzazione
            let offset = bn[0];
            bn.iter_mut().for_each(|v| *v -= offset);

            bet[i] = bn;
            bo = bn;
        }
    }

    /// Generazione delle metriche APP e decoding
    fn app_metric_generation(
        &self,
        len: usize,
        parity1: &[f64],
        parity2: &[f64],
        systematic: &[f64],
        extrinsic: &[f64],
        alf: &Vec<[f64; 4]>,
        bet: &Vec<[f64; 4]>,
        app: &mut Vec<[f64; 2]>,
        dec: &mut Vec<i32>,
    ) {
        for i in 0..len {
            let gp = [parity1[i], parity2[i]];
            let gsys = [systematic[i], extrinsic[i]];

            // Calcolo delle metriche APP
            let mut app0 = f64::NEG_INFINITY;
            let mut app1 = f64::NEG_INFINITY;

            for s in 0..4 {
                let (sp, x) = self.successor_state_and_output(s);
                app0 = app0.max(alf[i][s] + gp[x[0]] + gsys[0] + bet[i + 1][sp[0]]);
                app1 = app1.max(alf[i][s] + gp[x[1]] + gsys[1] + bet[i + 1][sp[1]]);
            }

            // Normalizzazione
            let max_val = app0.max(app1);
            app[i][0] = app0 - max_val;
            app[i][1] = app1 - max_val;

            // Decoding
            dec[i] = if app[i][0] >= app[i][1] { 1 } else { -1 };
        }
    }

    /// Calcolo di Alpha per uno stato
    fn alpha1(&self, prev: &[f64; 4], gp: &[f64; 2], gsys: &[f64; 2], state: usize) -> f64 {
        // Esempio di calcolo per stato
        match state {
            0 => prev[0] + gp[0] + gsys[0],
            1 => prev[1] + gp[1] + gsys[1],
            2 => prev[2] + gp[0] + gsys[1],
            _ => prev[3] + gp[1] + gsys[0],
        }
    }

    /// Calcolo di Beta per uno stato
    fn beta1(&self, next: &[f64; 4], gp: &[f64; 2], gsys: &[f64; 2], state: usize) -> f64 {
        // Esempio di calcolo per stato
        match state {
            0 => next[0] + gp[0] + gsys[0],
            1 => next[1] + gp[1] + gsys[1],
            2 => next[2] + gp[0] + gsys[1],
            _ => next[3] + gp[1] + gsys[0],
        }
    }

    /// Determinazione dello stato successivo e dell'output
    fn successor_state_and_output(&self, state: usize) -> ([usize; 2], [usize; 2]) {
        match state {
            0 => ([0, 2], [0, 1]),
            1 => ([0, 2], [1, 0]),
            2 => ([1, 3], [0, 1]),
            _ => ([1, 3], [1, 0]),
        }
    }
}
