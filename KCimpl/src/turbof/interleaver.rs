pub struct Interleaver{

}

impl Interleaver{

    pub fn new()-> Self{
        Self{

        }
    }

//ern is the u, in the previous function
pub fn mapint<T>(ls: usize, ern: Vec<T>, perm2: Vec<i32>) -> Vec<T>
where
    T: Clone, // T deve implementare il trait Clone
{
    let mut out = Vec::new();
    let num = perm2.len();  // lunghezza di perm2
    let parse = (ls as f64 / num as f64).floor() as usize;  // numero di blocchi di dati
    let parse = if parse == 0 { 1 } else { parse };  // Impostazione del numero minimo di blocchi
    let mut t: Vec<T> = Vec::new();

    for j in 0..parse {
        let mut t2 = ern[(num * j)..(num * (j + 1))].to_vec();  // estrai il blocco t2
        if t2.iter().all(|x| *x == t2[0]) {  // Verifica se t2 è tutto uguale (equivalente a sum == len)
            out.extend_from_slice(&t2);  // Aggiungi t2 a out
        } else {
            for l in 0..num {
                if perm2[l] == 1 {
                    if 2 * (l / 2) < l {  // l è dispari
                        out.push(t2[0].clone());  // Genera l'output
                        t = t2[1..(1 + num - l)].to_vec();  // Riprova la permutazione
                        t2 = Vec::new();
                    } else {  // l è pari
                        out.push(t[0].clone());  // Genera l'output
                        t2 = t[1..(1 + num - l)].to_vec();  // Riprova la permutazione
                    }
                } else if perm2[l] == 2 {
                    if 2 * (l / 2) < l {  // l è dispari
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
                    if 2 * (l / 2) < l {  // l è dispari
                        out.push(t2[perm2[l] as usize].clone());  // Genera l'output
                        let mut t = t2[1..perm2[l] as usize - 1].to_vec();
                        t.push(t2[0].clone());
                        t.extend_from_slice(&t2[(perm2[l] as usize)..(1 + num - l)]);
                        t2 = Vec::new();
                    } else {  // l è pari
                        out.push(t[perm2[l] as usize].clone());  // Genera l'output
                        t2 = t[1..perm2[l] as usize - 1].to_vec();
                        t2.push(t[0].clone());
                        t2.extend_from_slice(&t[(perm2[l] as usize)..(1 + num - l)]);
                    }
                }
            }

            // Permutazione finale per l'interleaver di lunghezza 4 (fisso)
            if 2 * (num / 2) < num {  // num è dispari
                out.push(t[3].clone());
                out.push(t[2].clone());
                out.push(t[0].clone());
                out.push(t[1].clone());
            } else {  // num è pari
                out.push(t2[3].clone());
                out.push(t2[2].clone());
                out.push(t2[0].clone());
                out.push(t2[1].clone());
            }
        }
        // Resetta t e t2 per continuare con il prossimo blocco
        t.clear();
        t2.clear();
    }

    out
}

    pub fn mapdint(ls: usize, ern: Vec<i32>, perm2: Vec<usize>) -> Vec<i32> {
        let mut out = Vec::new(); // Output inizializzato vuoto
        let num = perm2.len(); // Lunghezza di perm2
        let mut parse = ls / num; // Numero di blocchi di dati
        if parse == 0 {
            parse = 1; // Imposta almeno un blocco
        }

        for j in 0..parse {
            // Estrai il blocco corrente
            let mut t2 = ern[(num * j)..(num * (j + 1))].to_vec();
            if t2.iter().sum::<i32>() == t2.len() as i32 {
                // Se tutti gli elementi in t2 sono 1, aggiungi direttamente a out
                out.extend_from_slice(&t2);
            } else {
                let mut t = Vec::new(); // Inizializza t come vettore vuoto

                for l in 0..num {
                    if perm2[l] == 1 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[0]); // Genera output
                            t = t2[1..(1 + num - l)].to_vec(); // Ridimensiona il vettore
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[0]); // Genera output
                            t2 = t[1..(1 + num - l)].to_vec(); // Ridimensiona il vettore
                            t.clear();
                        }
                    } else if perm2[l] == 2 {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[1]); // Genera output
                            t = vec![t2[0]]
                                .into_iter()
                                .chain(t2[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[1]); // Genera output
                            t2 = vec![t[0]]
                                .into_iter()
                                .chain(t[2..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    } else {
                        if 2 * (l / 2) < l {
                            // l è dispari, lavora con t2
                            out.push(t2[perm2[l] - 1]); // Genera output
                            t = t2[1..perm2[l] - 1]
                                .to_vec()
                                .into_iter()
                                .chain(vec![t2[0]])
                                .chain(t2[perm2[l]..(1 + num - l)].iter().cloned())
                                .collect();
                            t2.clear();
                        } else {
                            // l è pari, lavora con t
                            out.push(t[perm2[l] - 1]); // Genera output
                            t2 = t[1..perm2[l] - 1]
                                .to_vec()
                                .into_iter()
                                .chain(vec![t[0]])
                                .chain(t[perm2[l]..(1 + num - l)].iter().cloned())
                                .collect();
                            t.clear();
                        }
                    }
                }

                // Trasposizioni finali per interleaver di lunghezza 4
                if 2 * (num / 2) < num {
                    // num dispari
                    out.push(t[2]);
                    out.push(t[3]);
                    out.push(t[1]);
                    out.push(t[0]);
                } else {
                    // num pari
                    out.push(t2[2]);
                    out.push(t2[3]);
                    out.push(t2[1]);
                    out.push(t2[0]);
                }
            }
            // Resetta t e t2 per il prossimo blocco
            t2.clear();
        }

        out
    }

    pub fn invperm(perm2: Vec<usize>) -> Vec<usize> {
        let num = perm2.len();
        let mut vec: Vec<usize> = (1..=num).collect(); // Crea un vettore [1, 2, ..., num]
        let mut nperm = Vec::new(); // Output per la permutazione intermedia

        let mut vec2 = Vec::new(); // Vettore temporaneo vuoto

        // Loop principale per generare la permutazione
        for l in 0..num {
            if perm2[l] == 1 {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[0]); // Genera output
                    vec2 = vec[1..(num - l)].to_vec(); // Ridimensiona `vec`
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[0]); // Genera output
                    vec = vec2[1..(num - l)].to_vec(); // Ridimensiona `vec2`
                    vec2.clear();
                }
            } else if perm2[l] == 2 {
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
            } else if perm2[l] == num - l {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[perm2[l] - 1]); // Genera output
                    vec2 = vec[1..perm2[l] - 1].to_vec();
                    vec2.insert(0, vec[0]);
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[perm2[l] - 1]); // Genera output
                    vec = vec2[1..perm2[l] - 1].to_vec();
                    vec.insert(0, vec2[0]);
                    vec2.clear();
                }
            } else {
                if 2 * (l / 2) < l {
                    // l dispari, lavora con `vec`
                    nperm.push(vec[perm2[l] - 1]); // Genera output
                    vec2 = vec[1..perm2[l] - 1]
                        .iter()
                        .cloned()
                        .chain(vec.iter().skip(perm2[l]))
                        .collect();
                    vec2.insert(0, vec[0]);
                    vec.clear();
                } else {
                    // l pari, lavora con `vec2`
                    nperm.push(vec2[perm2[l] - 1]); // Genera output
                    vec = vec2[1..perm2[l] - 1]
                        .iter()
                        .cloned()
                        .chain(vec2.iter().skip(perm2[l]))
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

