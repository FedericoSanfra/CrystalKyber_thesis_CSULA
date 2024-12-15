use crate::turbof::utils::{alpha1, beta1};
pub struct SISODecoder{

}

impl SISODecoder{

    pub fn new()->Self{
        Self{

        }
    }

    pub fn decode(
        &self,
        ls: usize,
        u: Vec<i32>,
        gamry11: &[f64],
        gamry12: &[f64],
        gamsys1: &[f64],
        gamsys2: &[f64],
    ) -> (Vec<Vec<f64>>, Vec<i32>, i32){

        ///ALPHA RECURSION
        let mut alf = vec![vec![0.0; ls+2]; 4];
        let mut ao = vec![0.0, -100.0, -100.0, -100.0]; // Inizializza alpha al passo zero.

        // Loop principale per aggiornare alpha fino al passo ls.
        for i in 0..ls {
            let mut an = vec![0.0; 4];
            let gp = vec![gamry11[i], gamry12[i]];
           // //println!("gamsys2 {:?}", gamsys2);
            let gsys = vec![gamsys1[i], gamsys2[i]];

            // Calcolo delle nuove alphas.
            for j in 0..4 {
                an[j] = alpha1(&ao, &gp, &gsys, j);
            }

            // Normalizzazione: importa solo la dimensione relativa.
            for j in 1..4 { //an(i+1) in matlab
                an[j] -= an[0];
            }
            an[0] = 0.0;

            // Aggiorna alf e prepara ao per la prossima iterazione.
            for j in 0..4{
                alf[j][i]=an[j];
            }

            ao = an.clone();
        }

        // Terminazione del trellis.
        for i in 0..2 {
            let mut an = vec![0.0; 4];
            let gp = vec![gamry11[ls + i], gamry12[ls + i]];
            let gsys = vec![gamsys1[ls + i], gamsys2[ls + i]];

            // Calcolo delle nuove alphas per la terminazione.
            for j in 0..4 {
                an[j] = alpha1(&ao, &gp, &gsys, j);
            }

            // Applicazione delle regole di terminazione.
            an[1] -= an[0];
            an[0] = 0.0;
            an[2] = -100.0;
            an[3] = -100.0;
            if i == 1 {
                an[1] = -100.0;
            }

            // Aggiorna alf e prepara ao per la prossima iterazione.
            // Assuming alf is a Vec<Vec<f64>> and An is a Vec<f64> with 4 elements
            for j in 0..4 {
                alf[j][ls + i] = an[j];
            }



            ao = an.clone();
        }


        ///BETA RECURSION

        // Beta recursion
        let parse = f64::floor(((ls + 2) / 4) as f64) - f64::ceil(((17 + 3) / 4) as f64); // Calcola il numero di blocchi
        let mut bet = vec![vec![0.0; ls+2]; 4]; // Inizializza il vettore bet

        // Ciclo principale per calcolare i betas con il loop di parse
        for i in 0..parse as usize {
            let d = 20;
            let gp1 = &gamry11[(4 * i)..(4 * i + 20)];
            let gp2 = &gamry12[(4 * i)..(4 * i + 20)];
            let gsys1 = &gamsys1[(4 * i)..(4 * i + 20)];
            let gsys2 = &gamsys2[(4 * i)..(4 * i + 20)];
            let gsys=[gsys1, gsys2];
            let gp=[gp1, gp2];

            let mut bo = vec![0.0, 0.0, 0.0, 0.0]; // inizializzazione dei betas
            let b= beta1(gp, gsys, d, bo);

            // Assegna i betas calcolati

            for j in 0..4 {
                for k in 0..4 {
                    bet[j][4 * (i - 1) + k] = b[j][k];
                }
            }
        }

        // Gestione del caso per il resto dei betas
        let d2 = (ls + 2 - parse as usize * 4) - 1; // Calcola l'ultima iterazione di beta
        let mut bo = vec![0.0, -100.0, -100.0, -100.0]; // Stato terminale
        let gp1 = &gamry11[(4 * parse as usize)..(ls + 2)];
        let gp2 = &gamry12[(4 * parse as usize)..(ls + 2)];
        let gsys1 = &gamsys1[(4 * parse as usize)..(ls + 2)];
        let gsys2 = &gamsys2[(4 * parse as usize)..(ls + 2)];
        let gsys=[gsys1, gsys2];
        let gp=[gp1, gp2];

        let b= beta1(gp, gsys, d2, bo);

        //println!(" bet matrix {:?}", bet);
        //println!(" b matrix {:?}", b);
        // Salvataggio dei risultati nel vettore bet
        for i in 0..4 {
            for j in 4 * parse as usize  ..(ls + 1 ) {
                //println!(" indexs i {:?} j {:?}", i, j);
                bet[i][j] = b[i][j - 4 * parse as usize];
            }
        }



        // App metric generation
        let mut app = vec![vec![0.0; ls + 2]; 2]; // Inizializza la variabile app
        let mut dec = vec![-1; ls]; // Inizializza il vettore dec
        let mut sig = vec![vec![0.0; 2]; 4]; // Inizializza sig

        for i in 1..(ls + 1) {
            for s in 0..4 {
                let (mut sp1, mut sp2, mut x1, mut x2) = (0, 0, 0, 0); // Inizializzazione delle variabili
                match s {
                    0 => {
                        sp1 = 0;
                        x1 = 0;
                        sp2 = 1;
                        x2 = 1;
                    }
                    1 => {
                        sp1 = 3;
                        x1 = 1;
                        sp2 = 2;
                        x2 = 0;
                    }
                    2 => {
                        sp1 = 1;
                        x1 = 0;
                        sp2 = 0;
                        x2 = 1;
                    }
                    _ => {
                        sp1 = 2;
                        x1 = 1;
                        sp2 = 3;
                        x2 = 0;
                    }
                }

                // Calcola sig
                sig[s][0] = alf[s][i - 1] + gp[x1][i] + gsys[0][i] + bet[sp1][i];
                sig[s][1] = alf[s][i - 1] + gp[x2][i] + gsys[1][i] + bet[sp2][i];
            }

            // Normalizzazione
            let c1 = sig[0..4]
                .iter()
                .map(|x| x[0]) // Assumi che ogni `x` sia un `Vec<f64>` o simile
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(); // Restituisce il massimo valore di tipo `f64`

            let c2 = sig[0..4]
                .iter()
                .map(|x| x[1]) // Accede al secondo elemento di ogni sotto-vettore
                .max_by(|a, b| a.partial_cmp(b).unwrap()) // Trova il massimo valore
                .unwrap(); // Restituisce il massimo come `f64`

            for s in 0..4 {
                sig[s][0] -= c1;
                sig[s][1] -= c2;
            }

            // Calcolo di app
            app[0][i] = c1 + sig[0..4].iter().map(|x| x[0]).fold(0.0, |acc, v| acc + v.exp()).ln();
            app[1][i] = c2 + sig[0..4].iter().map(|x| x[1]).fold(0.0, |acc, v| acc + v.exp()).ln();
        }

        // Impostazione iniziale per il primo passo della trellis
        app[0][0] = gp[0][0] + gsys[0][0] + bet[0][0];
        app[1][0] = gp[1][0] + gsys[1][0] + bet[1][0];

        // Thresholding
        for i in 0..ls {
            if app[0][i] >= app[1][i] {
                dec[i] = 1;
            } else {
                dec[i] = -1;
            }
        }

        // Calcolo dell'errore (count)
        let mut count = 0;
        for i in 0..ls {
            if dec[i] != u[i] {
                count += 1;
            }
        }

        // I risultati sono: app, dec, e count
        (app, dec, count)
    }
}