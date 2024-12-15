use std::fmt::Debug;
use std::iter::Sum;




pub struct Interleaver{}



impl Interleaver{

    pub fn new()-> Self{
        Self{

        }
    }



    //ern is the u, in the previous function
pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32>
{
    let mut out = Vec::new();
    let num = perm2.len();  // lunghezza di perm2
    let parse = (ls as f64 / num as f64).floor() as usize;  // numero di blocchi di dati
    let parse = if parse == 0 { 1 } else { parse };  // Impostazione del numero minimo di blocchi
    let mut t = Vec::with_capacity(perm2.len());

    for j in 1..=parse {
        let mut t2 = ern[(num * (j-1))..(num * (j))].to_vec();  // estrai il blocco t2
        let mut sum=0;
        for y in 0..t2.len(){
            sum=t2[y]  +sum;
        }
        if sum as usize== t2.len() {  // Verifica se t2 è tutto uguale (equivalente a sum == len)
            out.extend_from_slice(&t2);  // Aggiungi t2 a out
        } else {
            for l in 1..=num {
                if perm2[l-1] == 1 {
                    if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                        out.push(t2[0].clone());  // Genera l'output
                        let mut tmp =Vec::new();
                        for n in (1..1+num-l){
                            tmp.push(t2[n as usize].clone());
                        }
                        t = tmp;  // Riprova la permutazione
                        t2 = Vec::new();
                    } else {  // l è pari
                        println!("t vettore {:?}", t);
                        out.push(t[0].clone());  // Genera l'output
                        let mut tmp=Vec::new();
                        for n in (1..1+num - l){
                            tmp.push(t[n as usize].clone());
                        }
                        t2 = tmp;  // Riprova la permutazione
                    }
                } else if perm2[l-1] == 2 {
                    if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                        out.push(t2[1].clone());  // Genera l'output
                        let mut t = vec![t2[0].clone()]
                            .into_iter()
                            .chain(t2[2..( 1 + num - l)].iter().cloned())
                            .collect::<Vec<_>>();  // Ridimensiona
                        t2 = Vec::new();
                    } else {  // l è pari
                        out.push(t[1].clone());  // Genera l'output
                        t2 = vec![t[0].clone()]
                            .into_iter()
                            .chain(t[2..(1 + num - l)].iter().cloned())
                            .collect::<Vec<_>>();  // Ridimensiona
                    }
                } else {
                    if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                        out.push(t2[perm2[l-1] as usize].clone());  // Genera l'output
                        let mut t = t2[1..perm2[l-1] as usize -1].to_vec();
                        t.push(t2[0].clone());
                        t.extend_from_slice(&t2[(perm2[l-1] as usize)..(1 + num - l)]);
                        t2 = Vec::new();
                    } else {  // l è pari
                        out.push(t[perm2[l-1] as usize -1 ].clone());  // Genera l'output
                        t2 = t[1..perm2[l-1] as usize - 1].to_vec();
                        t2.push(t[0].clone());
                        t2.extend_from_slice(&t[(perm2[l-1] as usize)..(1 + num - l)]);
                    }
                }
            }

            //println!(" t vettore fine {:?}", t);

            // Permutazione finale per l'interleaver di lunghezza 4 (fisso)
            // if 2 * (num / 2) < num {  // num è dispari
            //     out.push(t[3].clone());
            //     out.push(t[2].clone());
            //     out.push(t[0].clone());
            //     out.push(t[1].clone());
            // } else {  // num è pari
            //     out.push(t2[3].clone());
            //     out.push(t2[2].clone());
            //     out.push(t2[0].clone());
            //     out.push(t2[1].clone());
            // }
        }
        // Resetta t e t2 per continuare con il prossimo blocco
        t.clear();
        t2.clear();
    }

    out
}

    pub fn mapint_f64(ls: usize, ern: Vec<f64>, perm2: Vec<i32>) -> Vec<f64> {
        let mut out = Vec::new();
        let num = perm2.len();  // lunghezza di perm2
        let parse = (ls as f64 / num as f64).floor() as usize;  // numero di blocchi di dati
        let parse = if parse == 0 { 1 } else { parse };  // Impostazione del numero minimo di blocchi
        let mut t = Vec::with_capacity(perm2.len());

        for j in 1..=parse {
            let mut t2 = ern[(num * (j - 1))..(num * j)].to_vec();  // estrai il blocco t2
            let mut sum = 0.0;
            for y in 0..t2.len() {
                sum = t2[y] + sum;
            }
            if sum as usize == t2.len() {  // Verifica se t2 è tutto uguale (equivalente a sum == len)
                out.extend_from_slice(&t2);  // Aggiungi t2 a out
            } else {
                for l in 1..=num {
                    if perm2[l - 1] == 1 {
                        if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                            out.push(t2[0].clone());  // Genera l'output
                            let mut tmp = Vec::new();
                            for n in (1..1 + num - l) {
                                tmp.push(t2[n as usize - 1].clone());
                            }
                            t = tmp;  // Riprova la permutazione
                            t2 = Vec::new();
                        } else {  // l è pari
                            println!("t vettore {:?}", t);
                            out.push(t[0].clone());  // Genera l'output
                            let mut tmp = Vec::new();
                            for n in (1..1 + num - l) {
                                tmp.push(t[n as usize].clone());
                            }
                            t2 = tmp;  // Riprova la permutazione
                        }
                    } else if perm2[l - 1] == 2 {
                        if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                            out.push(t2[1].clone());  // Genera l'output
                            let mut t = vec![t2[0].clone()]
                                .into_iter()
                                .chain(t2[2..(1 + num - l)].iter().cloned())
                                .collect::<Vec<_>>();  // Ridimensiona
                            t2 = Vec::new();
                        } else {  // l è pari
                            out.push(t[1].clone());  // Genera l'output
                            t2 = vec![t[0].clone()]
                                .into_iter()
                                .chain(t[2..(1 + num - l)].iter().cloned())
                                .collect::<Vec<_>>();  // Ridimensiona
                        }
                    } else {
                        if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
                            out.push(t2[perm2[l - 1] as usize - 1].clone());  // Genera l'output
                            let mut t = t2[1..perm2[l - 1] as usize - 1].to_vec();
                            t.push(t2[0].clone());
                            t.extend_from_slice(&t2[(perm2[l - 1] as usize)..(1 + num - l)]);
                            t2 = Vec::new();
                        } else {  // l è pari
                            out.push(t[perm2[l - 1] as usize - 1].clone());  // Genera l'output
                            t2 = t[1..perm2[l - 1] as usize - 1].to_vec();
                            t2.push(t[0].clone());
                            t2.extend_from_slice(&t[(perm2[l - 1] as usize)..(1 + num - l)]);
                        }
                    }
                }
            }
            // Resetta t e t2 per continuare con il prossimo blocco
            t.clear();
            t2.clear();
        }

        out
    }

    pub fn mapdint(ls: usize, ern: Vec<i32>, perm2: Vec<usize>) -> Vec<i32>
    {
        let mut out = Vec::new();
        let num = perm2.len(); // Lunghezza di perm2
        let mut parse = ls / num; // Numero di blocchi di dati
        if parse == 0 {
            parse = 1; // Imposta almeno un blocco
        }

        for j in 1..=parse {
            // Estrai il blocco corrente
            let mut t2 = ern[(num * (j - 1))..(num * j)].to_vec();
            let mut sum=0;
            for y in 0..t2.len(){
                sum=t2[y] +sum;
            }
            if sum as usize== t2.len()  {
                // Controlla se t2 contiene solo elementi uguali a Default
                out.extend_from_slice(&t2);
            } else {
                let mut t= Vec::new();

                for l in 1..=num {
                    if perm2[l - 1] == 1 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[0]);
                            t = t2[1..(1 + num - l)].to_vec();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[0]);
                            t2 = t[1..(1 + num - l)].to_vec();
                            t.clear();
                        }
                    } else if perm2[l - 1] == 2 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[1]);
                            t = vec![t2[0]]
                                .into_iter()
                                .chain(t2[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[1]);
                            t2 = vec![t[0]]
                                .into_iter()
                                .chain(t[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    } else {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            let idx = perm2[l - 1] - 1;
                            out.push(t2[idx as usize]);
                            t = t2[1..idx as usize]
                                .iter()
                                .cloned()
                                .chain(vec![t2[0]])
                                .chain(t2[(idx as usize + 1)..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            let idx = perm2[l - 1] - 1;
                            out.push(t[idx as usize]);
                            t2 = t[1..idx as usize]
                                .iter()
                                .cloned()
                                .chain(vec![t[0]])
                                .chain(t[(idx as usize + 1)..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    }
                }

                // Trasposizioni finali per interleaver di lunghezza 4
                // if num % 2 != 0 {
                //     // num dispari
                //     out.push(t[2]);
                //     out.push(t[3]);
                //     out.push(t[0]);
                //     out.push(t[1]);
                // } else {
                //     // num pari
                //     out.push(t2[2]);
                //     out.push(t2[3]);
                //     out.push(t2[0]);
                //     out.push(t2[1]);
                // }
            }

            // Resetta t e t2 per il prossimo blocco
            t2.clear();
        }

        out
    }

    pub fn mapdint_f64(ls: usize, ern: Vec<f64>, perm2: Vec<usize>) -> Vec<f64> {
        let mut out = Vec::new();
        let num = perm2.len(); // Lunghezza di perm2
        let mut parse = ls / num; // Numero di blocchi di dati
        if parse == 0 {
            parse = 1; // Imposta almeno un blocco
        }

        for j in 1..=parse {
            // Estrai il blocco corrente
            let mut t2 = ern[(num * (j - 1))..(num * j)].to_vec();
            let mut sum = 0.0;
            for y in 0..t2.len() {
                sum = t2[y] + sum;
            }
            if sum as usize == t2.len() {
                // Controlla se t2 contiene solo elementi uguali a Default
                out.extend_from_slice(&t2);
            } else {
                let mut t = Vec::new();

                for l in 1..=num {
                    if perm2[l - 1] == 1 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[0]);
                            t = t2[1..(1 + num - l)].to_vec();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[0]);
                            t2 = t[1..(1 + num - l)].to_vec();
                            t.clear();
                        }
                    } else if perm2[l - 1] == 2 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[1]);
                            t = vec![t2[0]]
                                .into_iter()
                                .chain(t2[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[1]);
                            t2 = vec![t[0]]
                                .into_iter()
                                .chain(t[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    } else {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            let idx = perm2[l - 1] - 1;
                            out.push(t2[idx as usize]);
                            t = t2[1..idx as usize]
                                .iter()
                                .cloned()
                                .chain(vec![t2[0]])
                                .chain(t2[(idx as usize + 1)..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            let idx = perm2[l - 1] - 1;
                            out.push(t[idx as usize]);
                            t2 = t[1..idx as usize]
                                .iter()
                                .cloned()
                                .chain(vec![t[0]])
                                .chain(t[(idx as usize + 1)..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    }
                }

                // Trasposizioni finali per interleaver di lunghezza 4
                // if num % 2 != 0 {
                //     // num dispari
                //     out.push(t[2]);
                //     out.push(t[3]);
                //     out.push(t[0]);
                //     out.push(t[1]);
                // } else {
                //     // num pari
                //     out.push(t2[2]);
                //     out.push(t2[3]);
                //     out.push(t2[0]);
                //     out.push(t2[1]);
                // }
            }

            // Resetta t e t2 per il prossimo blocco
            t2.clear();
        }

        out
    }



    // pub fn mapdint<T>(ls: usize, ern: Vec<T>, perm2: Vec<usize>) -> Vec<T>
    // where
    //     T: Copy + std::ops::Sub<Output = T> + std::iter::Sum + Default + std::cmp::PartialEq + Debug,  // Constraints per permettere l'uso di Sub e Sum
    // {
    //     let mut out = Vec::new();  // Output inizializzato vuoto
    //     let num = perm2.len();  // Lunghezza di perm2
    //     let mut parse = ls / num;  // Numero di blocchi di dati
    //     if parse == 0 {
    //         parse = 1;  // Imposta almeno un blocco
    //     }
    //
    //     for j in 1..=parse {
    //         // Estrai il blocco corrente
    //         let mut t2 = ern[(num * (j - 1))..(num * (j))].to_vec();
    //         if t2.iter().copied().sum::<T>() == T::default() {  // Se tutti gli elementi in t2 sono 0
    //             // Se tutti gli elementi in t2 sono 0, aggiungi direttamente a out
    //            // out = out.clone();
    //             out.extend_from_slice(&t2);
    //         } else {
    //             let mut t = Vec::new();  // Inizializza t come vettore vuoto
    //
    //             for l in 1..=num {
    //                 if perm2[l - 1] == 1 {
    //                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {
    //                         // l è dispari, lavora con t2
    //                         out.push(t2[0]);  // Genera output
    //                         t = t2[1..(1 + num - l)].to_vec();  // Ridimensiona il vettore
    //                         t2.clear();
    //                     } else {
    //                         // l è pari, lavora con t
    //                         out.push(t[0]);  // Genera output
    //                         t2 = t[1..(1 + num - l)].to_vec();  // Ridimensiona il vettore
    //                         t.clear();
    //                     }
    //                 } else if perm2[l - 1] == 2 {
    //                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {
    //                         // l è dispari, lavora con t2
    //                         out.push(t2[1]);  // Genera output
    //                         t = vec![t2[0]]
    //                             .into_iter()
    //                             .chain(t2[2..(1 + num - l)].iter().cloned())
    //                             .collect();
    //                         t2.clear();
    //                     } else {
    //                         // l è pari, lavora con t
    //                         out.push(t[1]);  // Genera output
    //                         t2 = vec![t[0]]
    //                             .into_iter()
    //                             .chain(t[2..(1 + num - l)].iter().cloned())
    //                             .collect();
    //                         t.clear();
    //                     }
    //                 } else {
    //                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64{
    //                         // l è dispari, lavora con t2
    //                         out.push(t2[perm2[l - 1] - 1]);  // Genera output
    //                         t = t2[1..perm2[l - 1] - 2]
    //                             .to_vec()
    //                             .into_iter()
    //                             .chain(vec![t2[0]])
    //                             .chain(t2[perm2[l - 1]..(1 + num - l)].iter().cloned())
    //                             .collect();
    //                         t2.clear();
    //                     } else {
    //                         println!("vettore t  mapdint{:?} num {:?} t2 {:?}", t, num, t2);
    //                         // l è pari, lavora con t
    //                         out.push(t[perm2[l - 1] - 1]);  // Genera output
    //                         t2 = t[1..perm2[l - 1] - 1]
    //                             .to_vec()
    //                             .into_iter()
    //                             .chain(vec![t[0]])
    //                             .chain(t[perm2[l - 1]..(1 + num - l)].iter().cloned())
    //                             .collect();
    //                         t.clear();
    //                     }
    //                 }
    //             }
    //
    //             // Trasposizioni finali per interleaver di lunghezza 4
    //             //     if 2 * (num / 2) < num {
    //             //         // num dispari
    //             //         out.push(t[2]);
    //             //         out.push(t[3]);
    //             //         out.push(t[1]);
    //             //         out.push(t[0]);
    //             //     } else {
    //             //         // num pari
    //             //         out.push(t2[2]);
    //             //         out.push(t2[3]);
    //             //         out.push(t2[1]);
    //             //         out.push(t2[0]);
    //             //     }
    //             // }
    //             // Resetta t e t2 per il prossimo blocco
    //             t2.clear();
    //         }
    //
    //
    //     }
    //
    //     out
    // }

    pub fn invperm(perm2: Vec<i32>) -> Vec<usize> {
        let num = perm2.len();
        let mut vec: Vec<usize> = (1..=num).collect(); // Crea un vettore [1, 2, ..., num]
        let mut nperm = Vec::new(); // Output per la permutazione intermedia

        let mut vec2 = Vec::new(); // Vettore temporaneo vuoto

        // Loop principale per generare la permutazione
        for l in 1..=num {
            if perm2[l - 1] == 1 {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[0]); // Genera output
                    vec2 = vec[1..(num + 1 - l)].to_vec(); // Ridimensiona `vec`
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[0]); // Genera output
                    vec = vec2[1..(num + 1 - l)].to_vec(); // Ridimensiona `vec2`
                    vec2.clear();
                }
            } else if perm2[l - 1] == 2 {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[1]); // Genera output
                    vec2 = vec.iter().skip(2).cloned().collect(); // Ridimensiona `vec`
                    vec2.insert(0, vec[0]);
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[1]); // Genera output
                    vec = vec2.iter().skip(2).cloned().collect(); // Ridimensiona `vec2`
                    vec.insert(0, vec2[0]);
                    vec2.clear();
                }
            } else if perm2[l - 1] == num as i32 - l as i32 + 1 {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[perm2[l as usize - 1] as usize - 1]); // Genera output
                    let mut tmp = Vec::new();
                    for n in (1..perm2[l - 1] - 1) {
                        tmp.push(vec[n as usize]);
                    }
                    vec2 = tmp; //operazione espansa su rust
                    vec2.insert(0, vec[0]);
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[perm2[l - 1] as usize - 1]); // Genera output
                    let mut tmp = Vec::new();
                    for n in (1..perm2[l - 1] as usize - 1) {
                        tmp.push(vec2[n as usize]);
                    }
                    vec = tmp; //logica espansa su rust per sicurezza maggiore
                    vec.insert(0, vec2[0]);
                    vec2.clear();
                }
            } else {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[perm2[l - 1] as usize - 1]); // Genera output
                    vec2 = vec[1..perm2[l - 1] as usize - 1]
                        .iter()
                        .cloned()
                        .chain(vec.iter().copied().skip(perm2[l - 1] as usize))
                        .collect();
                    vec2.insert(0, vec[0]);
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[perm2[l - 1] as usize - 1]); // Genera output
                    vec = vec2[1..perm2[l - 1] as usize - 1]
                        .iter()
                        .cloned()
                        .chain(vec2.iter().copied().skip(perm2[l - 1] as usize))
                        .collect();
                    vec.insert(0, vec2[0]);
                    vec2.clear();
                }
            }
        }

        // Calcolo della permutazione inversa
        let mut invp = vec![0; num];
        for i in 0..num {
            for j in i..num {
                if nperm[j] == i + 1 {
                    invp[i] = j - i + 1; // Registra la posizione
                    nperm.swap(i, j); // Scambia gli elementi
                    break;
                }
            }
        }

        invp
    }
}



// Per un altro tipo, ad esempio `f64`, possiamo implementare anche così:
// impl From<i32> for f64 {
//     fn from(value: i32) -> Self {
//         value as f64
//     }
// }




