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
        let mut alf = vec![vec![0.0; 4]; ls + 2];
        let mut ao = vec![0.0, -100.0, -100.0, -100.0]; // Inizializza alpha al passo zero.

        // Loop principale per aggiornare alpha fino al passo ls.
        for i in 0..ls {
            let mut an = vec![0.0; 4];
            let gp = vec![gamry11[i], gamry12[i]];
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
            alf[i] = an.clone();
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
            alf[ls + i] = an.clone();
            ao = an.clone();
        }


        ///BETA RECURSION

        // Beta recursion
        let parse = ((ls + 2) / 4) - ((17 + 3) / 4); // Calcola il numero di blocchi
        let mut bet = vec![vec![0.0; 4]; ls + 2]; // Inizializza il vettore bet

        // Ciclo principale per calcolare i betas con il loop di parse
        for i in 0..parse {
            let d = 20;
            let gp1 = &gamry11[(4 * i)..(4 * i + 20)];
            let gp2 = &gamry12[(4 * i)..(4 * i + 20)];
            let gsys1 = &gamsys1[(4 * i)..(4 * i + 20)];
            let gsys2 = &gamsys2[(4 * i)..(4 * i + 20)];
            let gsys=[gsys1, gsys2];
            let gp=[gp1, gp2];

            let mut bo = vec![0.0, 0.0, 0.0, 0.0]; // inizializzazione dei betas
            beta1(gp, gsys, d, bo);

            // Assegna i betas calcolati

            for j in 0..4 {
                for k in 0..4 {
                    bet[j][4 * (i - 1) + k] = bo[j][k];
                }
            }
        }

        // Gestione del caso per il resto dei betas
        let d2 = (ls + 2 - parse * 4) - 1; // Calcola l'ultima iterazione di beta
        let mut bo = vec![0.0, -100.0, -100.0, -100.0]; // Stato terminale
        let gp1 = &gamry11[(4 * parse)..(ls + 2)];
        let gp2 = &gamry12[(4 * parse)..(ls + 2)];
        let gsys1 = &gamsys1[(4 * parse)..(ls + 2)];
        let gsys2 = &gamsys2[(4 * parse)..(ls + 2)];
        let gsys=[gsys1, gsys2];
        let gp=[gp1, gp2];

        beta1(gp, gsys, d2, bo);

        // Salvataggio dei risultati nel vettore bet
        for i in 0..4 {
            for j in 4 * parse..(ls + 1) {
                bet[i][j] = bo[i][j - 4 * parse];
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
                        sp1 = 1;
                        x1 = 1;
                        sp2 = 2;
                        x2 = 2;
                    }
                    1 => {
                        sp1 = 4;
                        x1 = 2;
                        sp2 = 3;
                        x2 = 1;
                    }
                    2 => {
                        sp1 = 2;
                        x1 = 1;
                        sp2 = 1;
                        x2 = 2;
                    }
                    _ => {
                        sp1 = 3;
                        x1 = 2;
                        sp2 = 4;
                        x2 = 1;
                    }
                }

                // Calcola sig
                sig[s][0] = alf[s][i - 1] + gamry11[x1][i] + gamsys1[1][i] + bet[sp1][i];
                sig[s][1] = alf[s][i - 1] + gamry12[x2][i] + gamsys2[2][i] + bet[sp2][i];
            }

            // Normalizzazione
            let c1 = *sig[0..4].iter().map(|x| x[0]).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let c2 = *sig[0..4].iter().map(|x| x[1]).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            for s in 0..4 {
                sig[s][0] -= c1;
                sig[s][1] -= c2;
            }

            // Calcolo di app
            app[0][i] = c1 + sig[0..4].iter().map(|x| x[0]).fold(0.0, |acc, v| acc + v.exp()).ln();
            app[1][i] = c2 + sig[0..4].iter().map(|x| x[1]).fold(0.0, |acc, v| acc + v.exp()).ln();
        }

        // Impostazione iniziale per il primo passo della trellis
        app[0][0] = gamry11[0][0] + gamsys1[0][0] + bet[0][0];
        app[1][0] = gamry12[0][0] + gamsys2[0][0] + bet[1][0];

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