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