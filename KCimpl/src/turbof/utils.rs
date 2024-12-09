// Importa la libreria std per l'input/output e altre funzionalità di base
use std::io;

// Definizione dei polinomi per la generazione delle sequenze PN
const POL1: [[u8; 10]; 24] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 4, 5],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 3],
    [0, 0, 0, 0, 0, 0, 0, 2, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 4, 3, 5],
    [0, 0, 0, 0, 0, 0, 1, 0, 2, 1],
    [0, 0, 0, 0, 0, 0, 2, 0, 1, 1],
    [0, 0, 0, 0, 0, 0, 4, 0, 0, 5],
    [0, 0, 0, 0, 0, 1, 0, 1, 2, 3],
    [0, 0, 0, 0, 0, 2, 0, 0, 3, 3],
    [0, 0, 0, 0, 0, 4, 2, 1, 0, 3],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 3],
    [0, 0, 0, 0, 2, 1, 0, 0, 1, 3],
    [0, 0, 0, 0, 4, 0, 0, 0, 1, 1],
    [0, 0, 0, 1, 0, 0, 0, 2, 0, 1],
    [0, 0, 0, 2, 0, 0, 0, 0, 4, 7],
    [0, 0, 0, 4, 0, 0, 0, 0, 1, 1],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 5],
    [0, 0, 2, 0, 0, 0, 0, 0, 0, 3],
    [0, 0, 4, 0, 0, 0, 0, 0, 4, 1],
    [0, 1, 0, 0, 0, 0, 0, 2, 0, 7],
    [0, 2, 0, 0, 0, 0, 0, 0, 1, 1],
    [0, 4, 0, 0, 0, 0, 0, 1, 0, 7],
    [1, 0, 0, 0, 0, 0, 0, 0, 4, 7],
    [2, 0, 0, 0, 0, 0, 0, 0, 1, 1],
];

// Definizione degli altri polinomi (POL2 e POL3) simili a POL1 omessi per brevità

// Funzione principale per generare la sequenza PN
pub fn generate_pn_sequence(ndeg: usize, ngen: usize) -> Vec<i32> {
    let pol = match ngen {
        1 => &POL1[ndeg - 5],
        // Aggiungere le altre scelte per polinomi
        _ => panic!("Invalid generator selection."),
    };

    let mut coef = pol.to_vec();
    let mut polcn = Vec::new();
    let dig = [2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10];

    for n1 in 0..dig[ndeg - 5] {
        let val = coef[10 - n1 - 1];
        let mut polc = vec![0, 0, 0];
        match val {
            0 => {}
            1 => polc = vec![0, 0, 1 + 3 * n1],
            2 => polc = vec![0, 2 + 3 * n1, 0],
            3 => polc = vec![0, 2 + 3 * n1, 1 + 3 * n1],
            4 => polc = vec![3 + 3 * n1, 0, 0],
            5 => polc = vec![3 + 3 * n1, 0, 1 + 3 * n1],
            6 => polc = vec![3 + 3 * n1, 2 + 3 * n1, 0],
            _ => polc = vec![3 + 3 * n1, 2 + 3 * n1, 1 + 3 * n1],
        }
        polcn.extend_from_slice(&polc);
    }

    polcn.reverse();

    let mut out = vec![-1; ndeg + 1];
    out[ndeg] = 1;

    for j1 in 0..(2_usize.pow(ndeg as u32) - 1) {
        out.push(1);
        for j2 in 0..(ndeg + 1) {
            if polcn[ndeg + 2 - j2] != 0 {
                let z = polcn[ndeg + 2 - j2];
                out[j1 + ndeg + 1] *= out[j1 + ndeg + 1 - z];
            }
        }
    }

    out[ndeg + 2..(2_usize.pow(ndeg as u32) + ndeg)].to_vec()
}

/// Converte un vettore di bit (0 e 1) in un vettore di simboli (1.0 e -1.0)
pub fn bits_to_symbols(bits: &[i32]) -> Vec<f64> {
    bits.iter().map(|&bit| if bit == 1 { 1.0 } else { -1.0 }).collect()
}

/// Converte un vettore di simboli (1.0 e -1.0) in un vettore di bit (0 e 1)
pub fn symbols_to_bits(symbols: Vec<f64>) -> Vec<i32> {
    symbols.iter().map(|&symbol| if symbol == 1.0 { 1 } else { 0 }).collect()
}