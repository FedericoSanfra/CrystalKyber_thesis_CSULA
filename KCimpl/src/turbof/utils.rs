pub fn generate_pn_sequence(ndeg: usize, ngen: usize) -> Vec<i32> {
    // Coefficienti dei polinomi incorporati in Octale
    let pol1: Vec<Vec<i32>> = vec![ /* ... */ ];  // Usa la stessa definizione di pol1
    let pol2: Vec<Vec<i32>> = vec![ /* ... */ ];  // Usa la stessa definizione di pol2
    let pol3: Vec<Vec<i32>> = vec![ /* ... */ ];  // Usa la stessa definizione di pol3

    let dig = vec![2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10];

    let mut coef = vec![];
    if ngen == 1 {
        coef = pol1[ndeg - 5].clone();
    } else if ngen == 2 {
        coef = pol2[ndeg - 5].clone();
    } else {
        coef = pol3[ndeg - 5].clone();
    }

    let mut polcn: Vec<i32> = Vec::new();

    // Loop principale
    for n1 in 0..dig[ndeg - 5] {
        let val = coef[9 - n1];
        let polc = match val {
            0 => vec![0, 0, 0],
            1 => vec![0, 0, 1 + 3 * (n1 as i32)],
            2 => vec![0, 2 + 3 * (n1 as i32), 0],
            3 => vec![0, 2 + 3 * (n1 as i32), 1 + 3 * (n1 as i32)],
            4 => vec![3 + 3 * (n1 as i32), 0, 0],
            5 => vec![3 + 3 * (n1 as i32), 0, 1 + 3 * (n1 as i32)],
            6 => vec![3 + 3 * (n1 as i32), 2 + 3 * (n1 as i32), 0],
            _ => vec![3 + 3 * (n1 as i32), 2 + 3 * (n1 as i32), 1 + 3 * (n1 as i32)],
        };
        polcn.extend(polc);
    }

    let polcn = polcn.into_iter().rev().collect::<Vec<i32>>(); // inverti l'ordine

    // Inizializza LFSR
    let mut out: Vec<i32> = vec![1; ndeg];
    out.push(-1);

    // Genera la sequenza
    for j1 in 0..(2_usize.pow(ndeg as u32) - 1) {
        out.push(1);
        for j2 in 0..(ndeg + 1) {
            if polcn[ndeg + 1 - j2] != 0 {
                let z = polcn[ndeg + 1 - j2];
                // Correzione dell'operazione LFSR
                out[j1 + ndeg + 1] *= out[j1 + ndeg + 1 - z as usize];
            }
        }
    }

    out.split_off(ndeg + 1)
}

extern crate rand;
use rand::Rng;

pub fn generate_random_binary_input(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut binary_input = Vec::with_capacity(n);

    for _ in 0..n {
        let random_bit = rng.gen_range(0..2); // genera 0 o 1
        binary_input.push(random_bit);
    }

    binary_input
}

// Funzione per convertire un vettore di numeri binari in un vettore di 1 e -1
pub fn bits_to_levels(bin_vec: Vec<i32>) -> Vec<i32> {
    bin_vec.into_iter().map(|x| if x == 0 { -1 } else { 1 }).collect()
}

// Funzione per convertire un vettore di 1 e -1 in un vettore di numeri binari
pub fn levels_to_bits(levels: Vec<i32>) -> Vec<i32> {
    levels.into_iter().map(|x| if x == 1 { 1 } else { 0 }).collect()
}

///IN SISO DECODER

// Funzione per il calcolo di alpha usando il log-BCJR
pub fn alpha1(ao: &Vec<f64>, gp: &Vec<f64>, gsys: &Vec<f64>, s: usize) -> f64 {
    // Definizione delle variabili Sm e x
    let mut sm = vec![0; 2]; // Stati predecessori
    let mut x = vec![0; 2];  // Indici per le transizioni

    // Determinazione dei predecessori dello stato e dei relativi indici di transizione
    match s {
        1 => {
            sm[0] = 1; x[0] = 1;
            sm[1] = 3; x[1] = 2;
        },
        2 => {
            sm[0] = 3; x[0] = 1;
            sm[1] = 1; x[1] = 2;
        },
        3 => {
            sm[0] = 4; x[0] = 2;
            sm[1] = 2; x[1] = 1;
        },
        4 => {
            sm[0] = 2; x[0] = 2;
            sm[1] = 4; x[1] = 1;
        },
        _ => println!("Invalid state S: {} in alpha 1", s),
    }

    // Calcolo del termine di normalizzazione log-cosh
    let a1 = ao[sm[0] - 1] + gp[x[0] - 1] + gsys[0];
    let a2 = ao[sm[1] - 1] + gp[x[1] - 1] + gsys[1];
    let cosh_term = 0.5 * (a1 - a2);
    let log_cosh = cosh_term.cosh().ln();

    // Calcolo del valore di alpha aggiornato
    0.5 * (a1 + a2) + log_cosh
}


pub fn beta1( //funzione di backward recursion
    gp: [&[f64]; 2], //matrice [gp1, gp2]
    gsys: [&[f64]; 2], //matrice [sys1, sys2]
    d: usize, //delay parameter
    mut bo: Vec<f64>,
) -> Vec<Vec<f64>> {
    let mut b = vec![vec![0.0; d]; 4]; // Matrice B di output

    // Loop principale di ricorsione inversa
    for l in 0..d {
        // Loop di aggiornamento degli stati
        for s in 0..4 {
            // Successori dello stato S
            let (sp, x) = match s {
                0 => ([0, 1], [0, 1]), // Sp(1)=Splus(u=1), Sp(2)=Splus(u=-1)
                1 => ([3, 2], [1, 0]),
                2 => ([1, 0], [0, 1]),
                3 => ([2, 3], [1, 0]),
                _ => panic!("Invalid state S: {} in beta1", s),
            };

            // Calcolo di B(S, D-l)
            let a1 = bo[sp[0]] + gp[x[0]][d - l - 1] + gsys[0][d - l - 1];
            let a2 = bo[sp[1]] + gp[x[1]][d - l - 1] + gsys[1][d - l - 1];
            let cosh_term = 0.5 * (a1 - a2);
            let log_cosh = cosh_term.cosh().ln();

            b[s][d - l - 1] = 0.5 * (a1 + a2) + log_cosh;
        }

        // Inizializzazione per la prossima iterazione
        bo = b.iter().map(|row| row[d - l - 1]).collect();
    }

    // Normalizzazione della matrice B
    let first_row = b[0].clone();
    for row in b.iter_mut().skip(1) {
        for (i, val) in row.iter_mut().enumerate() {
            *val -= first_row[i];
        }
    }
    b[0] = vec![0.0; d];

    // Restituzione della matrice B
    b
}
