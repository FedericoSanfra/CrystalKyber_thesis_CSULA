use std::fmt::Debug;
use std::iter::Sum;




pub struct Interleaver{}



impl Interleaver{

    pub fn new()-> Self{
        Self{

        }
    }

    ///nuova versione chatgpt
    pub fn mapint(ls: usize, mut ern: Vec<i32>, mut perm2: Vec<i32>) -> Vec<i32> {
        // Aggiungi un valore fittizio all'inizio di ern e perm2 per emulare gli indici di MATLAB
        ern.insert(0, -1);
        perm2.insert(0, -1);

        let num = perm2.len() - 1; // Lunghezza effettiva di perm2
        let mut parse = (ls as f64 / num as f64).floor() as usize; // Numero di blocchi di dati
        if parse == 0 {
            parse = 1; // Assicurati che ci sia almeno un blocco
        }

        let mut out = Vec::new();

        for j in 1..=parse {
            // Estrai il blocco corrente (MATLAB: ern((num)*(j-1)+1:(num)*j))
            let start = num * (j - 1) + 1;
            let end = num * j;
            let mut t2 = ern[start..=end].to_vec();

            // Verifica se tutti gli elementi di t2 sono 1
            if t2.iter().sum::<i32>() == t2.len() as i32 {
                out.extend(&t2); // Aggiungi t2 direttamente a out
            } else {
                let mut t = Vec::new();

                for l in 1..=num {
                    if perm2[l] == 1 {
                        if 2 * (f64::floor((l / 2) as f64) as i32) < l as i32 {
                            // l dispari, lavora con t2
                            out.push(t2[0]);
                            if 1+num-l+1>t2.len(){

                            }else{
                                t = t2[1..( 1+num - l+1)].to_vec();
                            }

                            t2.clear();
                        } else {
                            // l pari, lavora con t
                            out.push(t[0]);
                            if 1+num -l+1> t.len(){

                            }else{
                                t2 = t[1..(1 + num - l + 1)].to_vec();
                            }

                            t.clear();
                        }
                    } else if perm2[l] == 2 {
                        if 2 * (f64::floor((l / 2) as f64) as i32) < l as i32  {
                            // l dispari, lavora con t2
                            out.push(t2[1]);
                            if 1+num-l+1>t2.len(){
                                t=vec![t2[0]];
                            }else{
                                t = vec![t2[0]]
                                    .into_iter()
                                    .chain(t2[2..(1 + num - l + 1)].iter().cloned())//ma incolla fino a che può???
                                    .collect();
                            }

                            t2.clear();
                        } else {
                            // l pari, lavora con t
                            out.push(t[1]);
                            t2 = vec![t[0]]
                                .into_iter()
                                .chain(t[2..(1 + num - l + 1)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    } else {
                        if 2 * (f64::floor((l / 2) as f64) as i32) < l as i32 {
                            // l dispari, lavora con t2
                            let idx = perm2[l] as usize;
                            out.push(t2[idx-1]);
                            if idx + 1 > 1 + num - l {
                                t = t2[1..idx].iter().cloned().chain(vec![t2[0]]).collect();
                            } else {
                                t = t2[1..idx]
                                    .iter()
                                    .cloned()
                                    .chain(vec![t2[0]])
                                    .chain(t2[idx..(1 + num - l + 1)].iter().cloned())
                                    .collect();
                            }
                            t2.clear();
                        } else {
                            // l pari, lavora con t
                            let idx = perm2[l] as usize;
                            out.push(t[idx-1]);
                            t2 = t[1..idx]
                                .iter()
                                .cloned()
                                .chain(vec![t[0]])
                                .chain(t[idx..(1 + num - l + 1)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    }
                }
            }
        }

        // Rimuovi il valore fittizio da out
        out.remove(0);

        out
    }


    ///modifiche prxof e chtagpt
    // pub fn mapint(ls: usize, mut ern: Vec<i32>, mut perm2: Vec<i32>) -> Vec<i32> {
    //     // Aggiungi un valore negativo all'inizio del vettore ern per allinearlo agli indici MATLAB
    //     ern.insert(0, i32::MIN);  // Si utilizza un valore che non verrà mai utilizzato nelle operazioni
    //     perm2.insert(0, 0);
    //
    //     let num = perm2.len() - 1; // Lunghezza effettiva di `perm2` (escludendo il placeholder)
    //
    //     //let num = perm2.len(); // Lunghezza di perm2
    //     if num == 0 || ern.is_empty() {
    //         return Vec::new(); // Ritorna un vettore vuoto se perm2 o ern sono vuoti
    //     }
    //
    //     let parse = (ls / num).max(1); // Calcola il numero di blocchi, garantendo almeno un ciclo
    //     let mut out = Vec::new();
    //
    //     for j in 1..=parse {
    //         // Estrai il blocco corrente (MATLAB: ern((num)*(j-1)+1:(num)*j))
    //         let start = num * (j - 1) + 1;
    //         let end = num * j + 1;
    //         let mut t2 = ern[start..end].to_vec();  // La slice di ern è basata sull'indice
    //
    //         // Se tutti gli elementi di t2 sono 1, aggiungili direttamente a out
    //         if t2.iter().sum::<i32>() == t2.len() as i32 {
    //             out.extend(t2);
    //         } else {
    //             let mut t = Vec::new();
    //             for l in 1..=num {
    //                 let perm = perm2[l - 1];  // Compensiamo l'indice da 0 a 1 per MATLAB
    //
    //                 if perm == 1 {
    //                     if 2 * (l / 2) < l {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(0));
    //                         t = t2.split_off(0);
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(0));
    //                         t2 = t.split_off(0);
    //                     }
    //                 } else if perm == 2 {
    //                     if 2 * (l / 2) < l {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(1));
    //                         t = vec![t2.remove(0)]
    //                             .into_iter()
    //                             .chain(t2.clone())
    //                             .collect();
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(1));
    //                         t2 = vec![t.remove(0)]
    //                             .into_iter()
    //                             .chain(t.clone())
    //                             .collect();
    //                     }
    //                 } else {
    //                     let idx = (perm - 1) as usize;
    //                     if 2 * (l / 2) < l {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(idx));
    //                         t = vec![t2.remove(0)]
    //                             .into_iter()
    //                             .chain(t2.clone())
    //                             .collect();
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(idx));
    //                         t2 = vec![t.remove(0)]
    //                             .into_iter()
    //                             .chain(t.clone())
    //                             .collect();
    //                     }
    //                 }
    //             }
    //
    //             // Trasposizione finale per un interleaver di lunghezza 4
    //             // if num % 2 != 0 {
    //             //     out.extend(t.into_iter().rev());
    //             // } else {
    //             //     out.extend(t2.clone().into_iter().rev());
    //             // }
    //         }
    //     }
    //
    //     // Rimuovi il valore negativo extra all'inizio di out prima di restituirlo
    //     out.remove(0);
    //
    //     out
    // }

    ///VERSIONE MODERNA CHATGPT, NO MODIFICHE PROF ANCORA
    // pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32> {
    //     let num = perm2.len(); // Lunghezza di perm2
    //     if num == 0 || ern.is_empty() {
    //         return Vec::new(); // Ritorna un vettore vuoto se perm2 o ern sono vuoti
    //     }
    //
    //     let parse = (ls / num).max(1); // Calcola il numero di blocchi, garantendo almeno un ciclo
    //     let mut out = Vec::new();
    //
    //     for j in 0..parse {
    //         // Estrai il blocco corrente
    //         let start = j * num;
    //         let end = start + num;
    //         let mut t2 = ern[start..end].to_vec();
    //
    //         // Se tutti gli elementi di t2 sono 1, aggiungili direttamente a out
    //         if t2.iter().sum::<i32>() == t2.len() as i32 {
    //             out.extend(t2);
    //         } else {
    //             let mut t = Vec::new();
    //             for (l, &perm) in perm2.iter().enumerate() {
    //                 let l = l + 1; // Compensa l'indicizzazione da 1 in MATLAB
    //                 if perm == 1 {
    //                     if l % 2 != 0 {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(0));
    //                         t = t2.split_off(0);
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(0));
    //                         t2 = t.split_off(0);
    //                     }
    //                 } else if perm == 2 {
    //                     if l % 2 != 0 {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(1));
    //                         t = vec![t2.remove(0)]
    //                             .into_iter()
    //                             .chain(t2.clone())
    //                             .collect();
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(1));
    //                         t2 = vec![t.remove(0)]
    //                             .into_iter()
    //                             .chain(t.clone())
    //                             .collect();
    //                     }
    //                 } else {
    //                     let idx = (perm - 1) as usize;
    //                     if l % 2 != 0 {
    //                         // l dispari, lavora con t2
    //                         out.push(t2.remove(idx));
    //                         t = vec![t2.remove(0)]
    //                             .into_iter()
    //                             .chain(t2.clone())
    //                             .collect();
    //                     } else {
    //                         // l pari, lavora con t
    //                         out.push(t.remove(idx));
    //                         t2 = vec![t.remove(0)]
    //                             .into_iter()
    //                             .chain(t.clone())
    //                             .collect();
    //                     }
    //                 }
    //             }
    //
    //             // Trasposizione finale per un interleaver di lunghezza 4
    //             // if num % 2 != 0 {
    //             //     out.extend(t.into_iter().rev());
    //             // } else {
    //             //     out.extend(t2.clone().into_iter().rev());
    //             // }
    //         }
    //     }
    //     println!(" out {:?}", out);
    //     out
    // }

    //chatgpt version
    //indici matlab + elemento mockato in posizione 0

    // pub fn mapint(ls: usize, mut ern: Vec<i32>, mut perm2: Vec<i32>) -> Vec<i32> {
    //     let mut out = Vec::new();
    //
    //     // Aggiungi un valore placeholder all'inizio dei vettori per replicare gli indici MATLAB (1-based)
    //     ern.insert(0, i32::MIN);
    //     perm2.insert(0, 0);
    //
    //     let num = perm2.len() - 1; // Lunghezza effettiva di `perm2` (escludendo il placeholder)
    //     let mut parse = (ls as f64 / num as f64).floor() as usize; // Numero di blocchi di dati
    //     if parse == 0 {
    //         parse = 1; // Imposta almeno un blocco
    //     }
    //
    //     for j in 1..=parse {
    //         let start = num * (j - 1) + 1;
    //         let end = num * j + 1;
    //
    //         if start >= ern.len() || end > ern.len() {
    //             //println!("Range non valido: start={} end={} ern_len={}", start, end, ern.len());
    //             continue;
    //         }
    //
    //         let mut t2 = ern[start..end].to_vec(); // Estrai il blocco corrente
    //
    //         if t2.iter().sum::<i32>() == t2.len() as i32 {
    //             out.extend(&t2); // Aggiungi `t2` direttamente a `out`
    //         } else {
    //             let mut t = Vec::new();
    //
    //             for l in 1..=num {
    //                 if l >= perm2.len() {
    //                     //println!("Indice `l` non valido: l={} perm2_len={}", l, perm2.len());
    //                     continue;
    //                 }
    //
    //                 if perm2[l] == 1 {
    //                     if 2 * (l / 2) < l {
    //                         // `l` dispari, lavora con `t2`
    //                         if t2.len() >= 2 {
    //                             out.push(t2[1]); // MATLAB: `out=[out,t2(1)]`
    //                             let range_end = 1 + num - l + 1;
    //                             if range_end <= t2.len() {
    //                                 t = t2[2..range_end].to_vec(); // MATLAB: `t=[t2(2:(1+num-l))]`
    //                             } else {
    //                                 //println!("Range non valido in `t2`: range_end={} t2_len={}", range_end, t2.len());
    //                             }
    //                         } else {
    //                             //println!("Accesso non valido a `t2` con len={}", t2.len());
    //                         }
    //                         t2.clear();
    //                     } else {
    //                         // `l` pari, lavora con `t`
    //                         if t.len() >= 2 {
    //                             out.push(t[1]); // MATLAB: `out=[out,t(1)]`
    //                             let range_end = 1 + num - l + 1;
    //                             if range_end <= t.len() {
    //                                 t2 = t[2..range_end].to_vec(); // MATLAB: `t2=[t(2:(1+num-l))]`
    //                             } else {
    //                                 //println!("Range non valido in `t`: range_end={} t_len={}", range_end, t.len());
    //                             }
    //                         } else {
    //                             //println!("Accesso non valido a `t` con len={}", t.len());
    //                         }
    //                         t.clear();
    //                     }
    //                 } else if perm2[l] == 2 {
    //                     if 2 * (l / 2) < l {
    //                         // `l` dispari, lavora con `t2`
    //                         if t2.len() >= 3 {
    //                             out.push(t2[2]); // MATLAB: `out=[out,t2(2)]`
    //                             let range_end = 1 + num - l + 1;
    //                             if range_end > 2 && range_end <= t2.len() {
    //                                 t = vec![t2[1]]
    //                                     .into_iter()
    //                                     .chain(t2[3..range_end].iter().cloned())
    //                                     .collect(); // MATLAB: `t=[t2(1),t2(3:1+num-l)]`
    //                             } else {
    //                                 //println!("Range non valido in `t2`: range_end={} t2_len={}", range_end, t2.len());
    //                             }
    //                         } else {
    //                             //println!("Accesso non valido a `t2` con len={}", t2.len());
    //                         }
    //                         t2.clear();
    //                     } else {
    //                         // `l` pari, lavora con `t`
    //                         if t.len() >= 3 {
    //                             out.push(t[2]); // MATLAB: `out=[out,t(2)]`
    //                             let range_end = 1 + num - l + 1;
    //                             if range_end > 2 && range_end <= t.len() {
    //                                 t2 = vec![t[1]]
    //                                     .into_iter()
    //                                     .chain(t[3..range_end].iter().cloned())
    //                                     .collect(); // MATLAB: `t2=[t(1),t(3:1+num-l)]`
    //                             } else {
    //                                 //println!("Range non valido in `t`: range_end={} t_len={}", range_end, t.len());
    //                             }
    //                         } else {
    //                             //println!("Accesso non valido a `t` con len={}", t.len());
    //                         }
    //                         t.clear();
    //                     }
    //                 } else {
    //                     if 2 * (l / 2) < l {
    //                         // `l` dispari, lavora con `t2`
    //                         if perm2[l] as usize >= 1 && perm2[l] < t2.len() as i32 && t2.len() > 2 {
    //                             out.push(t2[perm2[l] as usize]); // MATLAB: `out=[out,t2(perm2(l))]`
    //                             let range_start = 2;
    //                             let range_middle = perm2[l] as usize;
    //                             let range_end = 1 + num - l + 1;
    //                             if perm2[l]+1> (1 + num - l) as i32 {
    //                                 t = t2[2..perm2[l] as usize-1]
    //                                     .iter()
    //                                     .cloned()
    //                                     .chain(vec![t2[1]]).collect();
    //                             }else{
    //                                 t = t2[2..perm2[l] as usize-1]
    //                                     .iter()
    //                                     .cloned()
    //                                     .chain(vec![t2[1]])
    //                                     .chain(
    //                                         t2.get(perm2[l] as usize + 1..1+num-l)
    //                                             .unwrap_or(&[])
    //                                             .iter()
    //                                             .cloned(),
    //                                     )
    //                                     .collect();
    //                             }
    //                             // MATLAB: `t=[t2(2:perm2(l)-1),t2(1),t2(perm2(l)+1:1+num-l)]`
    //                         } else {
    //                             //println!(
    //                             //     "Accesso non valido a `t2` con perm2[l]={} t2_len={}",
    //                             //     perm2[l], t2.len()
    //                             // );
    //                         }
    //                         t2.clear();
    //                     } else {
    //                         // `l` pari, lavora con `t`
    //                         if perm2[l] as usize >= 1 && perm2[l] < t.len() as i32 && t.len() > 2 {
    //                             out.push(t[perm2[l] as usize]); // MATLAB: `out=[out,t(perm2(l))]`
    //                             let range_start = 2;
    //                             let range_middle = perm2[l] as usize;
    //                             let range_end = 1 + num - l + 1;
    //                             t2 = t[range_start..range_middle]
    //                                 .iter()
    //                                 .cloned()
    //                                 .chain(vec![t[1]])
    //                                 .chain(
    //                                     t.get(range_middle + 1..range_end)
    //                                         .unwrap_or(&[])
    //                                         .iter()
    //                                         .cloned(),
    //                                 )
    //                                 .collect(); // MATLAB: `t2=[t(2:perm2(l)-1),t(1),t(perm2(l)+1:1+num-l)]`
    //                         } else {
    //                             // println!(
    //                             //     "Accesso non valido a `t` con perm2[l]={} t_len={}",
    //                             //     perm2[l], t.len()
    //                             // );
    //                         }
    //                         t.clear();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //    out.remove(0);
    //     // Rimuovi il placeholder da `out`
    //     out
    // }

    //chat gpt version, indici di rust
    // pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32> {
    //     let mut out = Vec::new();
    //     let num = perm2.len();  // lunghezza di perm2
    //     let parse = (ls as f64 / num as f64).floor() as usize;  // numero di blocchi di dati
    //     let parse = if parse == 0 { 1 } else { parse };  // Impostazione del numero minimo di blocchi
    //     let mut t = Vec::with_capacity(perm2.len());
    //
    //     for j in 0..parse {
    //         let start = num * j;  // calcola l'indice di inizio per ogni blocco
    //         let end = num * (j + 1);  // calcola l'indice di fine per ogni blocco
    //         let mut t2 = ern[start..end].to_vec();  // estrai il blocco t2
    //         let mut sum = 0;
    //         for y in 0..t2.len() {
    //             sum = t2[y] + sum;
    //         }
    //         if sum as usize == t2.len() {
    //             out.extend_from_slice(&t2);  // Aggiungi t2 a out
    //         } else {
    //             for l in 0..num {
    //                 if perm2[l] == 1 {
    //                     if 2.0 * f64::floor((l + 1) as f64 / 2.0) < (l + 1) as f64 {
    //                         // l è dispari
    //                         out.push(t2[0]);  // Genera l'output
    //                         let mut tmp = Vec::new();
    //                         for n in 1..(num - l) {
    //                             tmp.push(t2[n as usize]);
    //                         }
    //                         t = tmp;  // Riprova la permutazione
    //                         t2.clear();
    //                     } else {
    //                         // l è pari
    //                         out.push(t[0]);  // Genera l'output
    //                         let mut tmp = Vec::new();
    //                         for n in 1..(num - l) {
    //                             tmp.push(t[n as usize]);
    //                         }
    //                         t2 = tmp;  // Riprova la permutazione
    //                     }
    //                 } else if perm2[l] == 2 {
    //                     if 2.0 * f64::floor((l + 1) as f64 / 2.0) < (l + 1) as f64 {
    //                         // l è dispari
    //                         out.push(t2[1]);  // Genera l'output
    //                         let mut t = vec![t2[0]]
    //                             .into_iter()
    //                             .chain(t2[2..(num - l)].iter().cloned())
    //                             .collect::<Vec<_>>();  // Ridimensiona
    //                         t2.clear();
    //                     } else {
    //                         // l è pari
    //                         out.push(t[1]);  // Genera l'output
    //                         t2 = vec![t[0]]
    //                             .into_iter()
    //                             .chain(t[2..(num - l)].iter().cloned())
    //                             .collect::<Vec<_>>();  // Ridimensiona
    //                     }
    //                 } else {
    //                     if 2.0 * f64::floor((l + 1) as f64 / 2.0) < (l + 1) as f64 {
    //                         // l è dispari
    //                         println!("perm 2 {:?}", perm2);
    //                         println!("t2 vec {:?}", t2);
    //                         out.push(t2[perm2[l] as usize]);  // Genera l'output
    //                         let mut t = t2[0..perm2[l] as usize].to_vec();
    //                         t.push(t2[0]);
    //                         t.extend_from_slice(&t2[(perm2[l] as usize)..(num - l)]);
    //                         t2.clear();
    //                     } else {
    //                         // l è pari
    //                         out.push(t[perm2[l] as usize ]);  // Genera l'output
    //                         t2 = t[1..perm2[l] as usize - 1].to_vec();
    //                         t2.push(t[0]);
    //                         t2.extend_from_slice(&t[(perm2[l] as usize)..(num - l)]);
    //                     }
    //                 }
    //             }
    //         }
    //
    //         // Resetta t e t2 per continuare con il prossimo blocco
    //         t.clear();
    //         t2.clear();
    //     }
    //
    //     out
    // }
    //


    //ern is the u, in the previous function
// pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32>
// {
//     let mut out = Vec::new();
//     let num = perm2.len();  // lunghezza di perm2
//     let parse = (ls as f64 / num as f64).floor() as usize;  // numero di blocchi di dati
//     let parse = if parse == 0 { 1 } else { parse };  // Impostazione del numero minimo di blocchi
//     let mut t = Vec::with_capacity(perm2.len());
//
//     for j in 1..=parse {
//         let mut t2 = ern[(num * (j-1))..(num * (j))].to_vec();  // estrai il blocco t2
//         let mut sum=0;
//         for y in 0..t2.len(){
//             sum=t2[y]  +sum;
//         }
//         if sum as usize== t2.len() {  // Verifica se t2 è tutto uguale (equivalente a sum == len)
//             out.extend_from_slice(&t2);  // Aggiungi t2 a out
//         } else {
//             for l in 1..=num {
//                 if perm2[l-1] == 1 {
//                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
//                         out.push(t2[0].clone());  // Genera l'output
//                         let mut tmp =Vec::new();
//                         for n in (1..1+num-l){
//                             tmp.push(t2[n as usize].clone());
//                         }
//                         t = tmp;  // Riprova la permutazione
//                         t2 = Vec::new();
//                     } else {  // l è pari
//                         println!("t vettore {:?}", t);
//                         out.push(t[0].clone());  // Genera l'output
//                         let mut tmp=Vec::new();
//                         for n in (1..1+num - l){
//                             tmp.push(t[n as usize].clone());
//                         }
//                         t2 = tmp;  // Riprova la permutazione
//                     }
//                 } else if perm2[l-1] == 2 {
//                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
//                         out.push(t2[1].clone());  // Genera l'output
//                         let mut t = vec![t2[0].clone()]
//                             .into_iter()
//                             .chain(t2[2..( 1 + num - l)].iter().cloned())
//                             .collect::<Vec<_>>();  // Ridimensiona
//                         t2 = Vec::new();
//                     } else {  // l è pari
//                         out.push(t[1].clone());  // Genera l'output
//                         t2 = vec![t[0].clone()]
//                             .into_iter()
//                             .chain(t[2..(1 + num - l)].iter().cloned())
//                             .collect::<Vec<_>>();  // Ridimensiona
//                     }
//                 } else {
//                     if 2.0 * f64::floor(l as f64 / 2.0) < l as f64 {  // l è dispari
//                         out.push(t2[perm2[l-1] as usize].clone());  // Genera l'output
//                         let mut t = t2[1..perm2[l-1] as usize -1].to_vec();
//                         t.push(t2[0].clone());
//                         t.extend_from_slice(&t2[(perm2[l-1] as usize)..(1 + num - l)]);
//                         t2 = Vec::new();
//                     } else {  // l è pari
//                         out.push(t[perm2[l-1] as usize -1 ].clone());  // Genera l'output
//                         t2 = t[1..perm2[l-1] as usize - 1].to_vec();
//                         t2.push(t[0].clone());
//                         t2.extend_from_slice(&t[(perm2[l-1] as usize)..(1 + num - l)]);
//                     }
//                 }
//             }
//
//             //println!(" t vettore fine {:?}", t);
//
//             // Permutazione finale per l'interleaver di lunghezza 4 (fisso)
//             // if 2 * (num / 2) < num {  // num è dispari
//             //     out.push(t[3].clone());
//             //     out.push(t[2].clone());
//             //     out.push(t[0].clone());
//             //     out.push(t[1].clone());
//             // } else {  // num è pari
//             //     out.push(t2[3].clone());
//             //     out.push(t2[2].clone());
//             //     out.push(t2[0].clone());
//             //     out.push(t2[1].clone());
//             // }
//         }
//         // Resetta t e t2 per continuare con il prossimo blocco
//         t.clear();
//         t2.clear();
//     }
//
//     out
// }

///MAPINT F64
pub fn mapint_f64(ls: usize, ern: Vec<f64>, perm2: Vec<i32>) -> Vec<f64> {
    let num = perm2.len(); // Lunghezza di perm2
    if num == 0 || ern.is_empty() {
        return Vec::new(); // Ritorna un vettore vuoto se perm2 o ern sono vuoti
    }

    let parse = (ls / num).max(1); // Calcola il numero di blocchi, garantendo almeno un ciclo
    let mut out = Vec::new();

    for j in 0..parse {
        // Estrai il blocco corrente
        let start = j * num;
        let end = start + num;
        let mut t2 = ern[start..end].to_vec();

        // Se tutti gli elementi di t2 sono 1, aggiungili direttamente a out
        if t2.iter().sum::<f64>() == t2.len() as f64 {
            out.extend(t2);
        } else {
            let mut t = Vec::new();
            for (l, &perm) in perm2.iter().enumerate() {
                let l = l + 1; // Compensa l'indicizzazione da 1 in MATLAB
                if perm == 1 {
                    if l % 2 != 0 {
                        // l dispari, lavora con t2
                        out.push(t2.remove(0));
                        t = t2.split_off(0);
                    } else {
                        // l pari, lavora con t
                        out.push(t.remove(0));
                        t2 = t.split_off(0);
                    }
                } else if perm == 2 {
                    if l % 2 != 0 {
                        // l dispari, lavora con t2
                        out.push(t2.remove(1));
                        t = vec![t2.remove(0)]
                            .into_iter()
                            .chain(t2.clone())
                            .collect();
                    } else {
                        // l pari, lavora con t
                        out.push(t.remove(1));
                        t2 = vec![t.remove(0)]
                            .into_iter()
                            .chain(t.clone())
                            .collect();
                    }
                } else {
                    let idx = (perm - 1) as usize;
                    if l % 2 != 0 {
                        // l dispari, lavora con t2
                        out.push(t2.remove(idx));
                        t = vec![t2.remove(0)]
                            .into_iter()
                            .chain(t2.clone())
                            .collect();
                    } else {
                        // l pari, lavora con t
                        out.push(t.remove(idx));
                        t2 = vec![t.remove(0)]
                            .into_iter()
                            .chain(t.clone())
                            .collect();
                    }
                }
            }

            // Trasposizione finale per un interleaver di lunghezza 4
            if num % 2 != 0 {
                out.extend(t.into_iter().rev());
            } else {
                out.extend(t2.clone().into_iter().rev());
            }
        }
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




