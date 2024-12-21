pub fn generate_pn_sequence(ndeg: usize, ngen: usize) -> Vec<i32> {
    // Coefficienti dei polinomi incorporati in Octale
    let pol1 = vec![[0,0,0,0,0,0,0,0,4,5],
[0,0,0,0,0,0,0,1,0,3],
[0,0,0,0,0,0,0,2,1,1],
[0,0,0,0,0,0,0,4,3,5],
[0,0,0,0,0,0,1,0,2,1],
[0,0,0,0,0,0,2,0,1,1],
[0,0,0,0,0,0,4,0,0,5],
[0,0,0,0,0,1,0,1,2,3],
[0,0,0,0,0,2,0,0,3,3],
[0,0,0,0,0,4,2,1,0,3],
[0,0,0,0,1,0,0,0,0,3],
[0,0,0,0,2,1,0,0,1,3],
[0,0,0,0,4,0,0,0,1,1],
[0,0,0,1,0,0,0,2,0,1],
[0,0,0,2,0,0,0,0,4,7],
[0,0,0,4,0,0,0,0,1,1],
[0,0,1,0,0,0,0,0,0,5],
[0,0,2,0,0,0,0,0,0,3],
[0,0,4,0,0,0,0,0,4,1],
[0,1,0,0,0,0,0,2,0,7],
[0,2,0,0,0,0,0,0,1,1],
[0,4,0,0,0,0,0,1,0,7],
[1,0,0,0,0,0,0,0,4,7],
[2,0,0,0,0,0,0,0,1,1]];  // Usa la stessa definizione di pol1

    let pol2= vec![[0,0,0,0,0,0,0,0,7,5],
[0,0,0,0,0,0,0,1,4,7],
[0,0,0,0,0,0,0,2,1,7],
[0,0,0,0,0,0,0,4,5,3],
[0,0,0,0,0,0,1,0,5,5],
[0,0,0,0,0,0,2,0,3,3],
[0,0,0,0,0,0,4,0,5,5],
[0,0,0,0,0,1,1,0,1,5],
[0,0,0,0,0,2,1,1,0,3],
[0,0,0,0,0,4,4,1,0,3],
[0,0,0,0,1,0,2,0,4,3],
[0,0,0,0,3,0,7,1,0,7],
[0,0,0,0,4,0,0,0,1,7],
[0,0,0,1,7,0,3,6,0,1],
[0,0,0,2,0,2,0,4,7,1],
[0,0,0,6,0,0,0,0,3,1],
[0,0,1,0,0,4,0,2,0,5],
[0,0,2,0,0,0,1,0,4,3],
[0,0,4,0,0,0,0,0,6,3],
[0,1,2,5,2,4,5,6,6,1],
[0,2,0,0,0,0,0,0,1,7],
[0,4,3,0,2,1,6,4,7,3],
[1,2,5,0,0,2,5,7,5,7],
[2,1,0,4,2,1,0,4,3,1]];  // Usa la stessa definizione di pol2


    let pol3= vec![ [0,0,0,0,0,0,0,0,6,7],
[0,0,0,0,0,0,0,1,5,5],
[0,0,0,0,0,0,0,2,3,5],
[0,0,0,0,0,0,0,7,0,3],
[0,0,0,0,0,0,1,5,5,3],
[0,0,0,0,0,0,2,0,4,7],
[0,0,0,0,0,0,5,0,0,7],
[0,0,0,0,0,1,1,2,7,1],
[0,0,0,0,0,2,2,2,0,3],
[0,0,0,0,0,4,0,1,2,3],
[0,0,0,0,1,1,0,0,1,3],
[0,0,0,0,2,0,1,7,3,5],
[0,0,0,0,6,0,0,0,1,3],
[0,0,0,1,1,1,5,7,0,1],
[0,0,0,3,6,1,0,3,5,3],
[0,0,0,4,4,4,2,2,3,5],
[0,0,1,1,1,1,1,1,1,5],
[0,0,2,2,2,2,2,2,2,3],
[0,0,4,1,2,2,4,4,4,5],
[0,1,1,3,7,6,3,0,6,3],
[0,2,0,0,0,1,4,7,3,1],
[0,4,7,3,1,6,7,5,4,5],
[1,0,3,7,5,3,0,2,4,1],
[2,0,2,0,0,0,6,0,3,1 ]];  // Usa la stessa definizione di pol3

    let dig = vec![2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10];

    let mut coef = vec![];
    if ngen == 1 {
        coef = Vec::from(pol1[ndeg - 5].clone());
    } else if ngen == 2 {
        coef = Vec::from(pol2[ndeg - 5].clone());
    } else {
        coef = Vec::from(pol3[ndeg - 5].clone());
    }

    let mut polcn: Vec<i32> = Vec::new();

    // Loop principale
    for n1 in 1..=dig[ndeg - 5] {
        let val = coef[10 - n1];
        let mut polc = match val {
            0 => vec![0, 0, 0],
            1 => vec![0, 0, 1 + 3 * (n1 as i32 -1)],
            2 => vec![0, 2 + 3 * (n1 as i32 -1), 0],
            3 => vec![0, 2 + 3 * (n1 as i32 -1 ), 1 + 3 * (n1 as i32 -1)],
            4 => vec![3 + 3 * (n1 as i32-1), 0, 0],
            5 => vec![3 + 3 * (n1 as i32-1), 0, 1 + 3 * (n1 as i32 -1)],
            6 => vec![3 + 3 * (n1 as i32-1), 2 + 3 * (n1 as i32 -1), 0],
            _ => vec![3 + 3 * (n1 as i32-1), 2 + 3 * (n1 as i32 -1), 1 + 3 * (n1 as i32 -1)],
        };
        let tmp=polcn.clone();
        polc.extend(tmp);
        polcn=polc.clone(); //ricorsivo
    }

    let polcn = polcn.into_iter().rev().collect::<Vec<i32>>(); // inverti l'ordine

    // Inizializza LFSR
    let mut out: Vec<i32> = Vec::new();
    out.insert(0,-1); //elemento mock
    for n1 in 1..=ndeg{
        out.push(1); //parto dall'elemento 1 anzichè 0
    }
    out.push(-1); //in posizione ndeg+1 fuori dal for cycle

    // Genera la sequenza
    for j1 in 1..=(2_usize.pow(ndeg as u32) - 1) {
        out.insert(j1+ndeg+1,1);
        for j2 in 1..=(ndeg + 1) {
            if polcn[ndeg + 1 - j2] != 0 {
                let z = polcn[ndeg + 1 - j2];
                // Correzione dell'operazione LFSR
                out[j1 + ndeg + 1] *= out[j1 + ndeg + 1 - z as usize];
            }
        }
    }

    //out.remove(0); //rimuovo l'elemento di mock
    let start_range=ndeg+2;
    let end_range=2_usize.pow(ndeg as u32) +ndeg;
    out[start_range..=end_range].to_vec()
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
        0 => {
            sm[0] = 0; x[0] = 0;
            sm[1] = 2; x[1] = 1;
        },
        1 => {
            sm[0] = 2; x[0] = 0;
            sm[1] = 0; x[1] = 1;
        },
        2 => {
            sm[0] = 3; x[0] = 1;
            sm[1] = 1; x[1] = 0;
        },
        3 => {
            sm[0] = 1; x[0] = 1;
            sm[1] = 3; x[1] = 0; //valori decrementati da matlab
        },
        _ => println!("Invalid state S: {} in alpha 1", s),
    }

    // Calcolo del termine di normalizzazione log-cosh

    let an=0.5*(ao[sm[0]]+gp[x[0]]+gsys[0]+ao[sm[1]]+gp[x[1]]) +gsys[1]+((0.5*(ao[sm[0]]+gp[x[0]]+gsys[0]-ao[sm[1]]-gp[x[1]]-gsys[1])).cosh().ln());


    // Calcolo del valore di alpha aggiornato
    an
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
            // let a1 = bo[sp[0]] + gp[x[0]][d - l - 1] + gsys[0][d - l - 1];
            // let a2 = bo[sp[1]] + gp[x[1]][d - l - 1] + gsys[1][d - l - 1];
            // let cosh_term = 0.5 * (a1 - a2);
            // let log_cosh = cosh_term.cosh().ln();
            //
            // b[s][d - l + 1 -1] = 0.5 * (a1 + a2) + log_cosh;
            // println!("s {:?} d-l+2 {:?}",s, d-l);
            // println!("gp {:?}", &gp[0].len());
            b[s][d-l-1]=0.5*(bo[sp[0]]+gp[x[0]][d-l]+gsys[0][d-l]+bo[sp[1]]+gp[x[1]][d-l]+gsys[1][d-l])+((0.5*(bo[sp[0]]+gp[x[0]][d-l]+gsys[0][d-l]-bo[sp[1]]-gp[x[1]][d-l]-gsys[1][d-l])).cosh()).ln();
        } //con d=20, d-l indice corrispondente a matlab

        // Inizializzazione per la prossima iterazione
        bo = b.iter().map(|row| row[d - l -1]).collect();
    }

    // Normalizzazione della matrice B
    let first_row = b[0].clone();
    for row in b.iter_mut().skip(1) {
        for (i, val) in row.iter_mut().enumerate() {
            *val -= first_row[i];
        }
    }
    b[0] = vec![0.0; d]; //clear of the first row

    // Restituzione della matrice B
    b
}



pub fn generate_binary_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..2)).collect()
}

/// Convert a vector of transpositions to a permutation vector for use as an interleaver.
pub fn transpositions_to_permutations(transpositions: Vec<i32>) -> Vec<usize> {
    let n = transpositions.len();
    let mut permutation = (0..n).collect::<Vec<_>>(); // Start with identity permutation

    for i in 0..n {
        permutation.swap(i, transpositions[i] as usize); // Apply each transposition
    }

    permutation
}

/// Applica una permutazione a un vettore di input.
///
/// # Parametri:
/// - `input`: Vettore di input da permutare.
/// - `permutations`: Vettore di permutazioni.
///
/// # Ritorno:
/// - Vettore permutato.
pub fn apply_permutation<T: Clone+ Default>(vector: Vec<T>, interleaver: Vec<usize>) -> Vec<T> {
    let mut interleaved: Vec<T> = vec![vector[0].clone(); vector.len()];
    for (i, &index) in interleaver.iter().enumerate() {
        interleaved[i] = vector[index].clone();
    }
    interleaved
}
// pub fn apply_permutation<T: Clone>(input: Vec<T>, permutations: Vec<usize>) -> Vec<T> {
//     let mut output = vec![input[0].clone(); input.len()]; // Vettore di output inizializzato
//     for (i, &p) in permutations.iter().enumerate() {
//         output[p - 1] = input[i].clone(); // Permutazione: posiziona `input[i]` nell'indice `p - 1`
//     }
//     output
// }

/// Ripristina il vettore originale a partire da un vettore permutato.
///
/// # Parametri:
/// - `permuted`: Vettore permutato.
/// - `permutations`: Vettore di permutazioni usato per permutare l'originale.
///
/// # Ritorno:
/// - Vettore originale.

pub fn reverse_permutation<T: Clone>(vector: Vec<T>, interleaver: Vec<usize>) -> Vec<T> {
    let mut deinterleaved = vec![vector[0].clone(); vector.len()];
    for (i, &index) in interleaver.iter().enumerate() {
        deinterleaved[index] = vector[i].clone();
    }
    deinterleaved
}

// pub fn reverse_permutation<T: Clone>(permuted: Vec<T>, permutations: Vec<usize>) -> Vec<T> {
//     let mut original = vec![permuted[0].clone(); permuted.len()]; // Vettore originale inizializzato
//     for (i, &p) in permutations.iter().enumerate() {
//         original[i] = permuted[p - 1].clone(); // Ripristina il valore originale
//     }
//     original
// }


