use crate::turbof::utils::symbols_to_bits;

pub struct Interleaver {
    permutation: Vec<usize>,
    inverse_permutation: Vec<usize>,
}

impl Interleaver {
    /// Crea un nuovo interleaver con il vettore di permutazione dato
    pub fn new(permutation: Vec<usize>) -> Self {
        let inverse_permutation = invperm(&permutation);

        Self {
            permutation,
            inverse_permutation,
        }
    }

    /// Applica l'interleaving ai dati
    pub fn interleave(&self, data: Vec<f64>) -> Vec<i32> {
        mapint(data.len(), &symbols_to_bits(data), &self.permutation)
    }

    /// Applica il deinterleaving ai dati
    pub fn deinterleave(&self, data: Vec<f64>) -> Vec<i32> {
        mapdint(data.len(), &symbols_to_bits(data), &self.inverse_permutation)
    }
}

// Implementazione della funzione invperm per calcolare la permutazione inversa
fn invperm(perm2: &[usize]) -> Vec<usize> {
    let num = perm2.len();
    let mut nperm = vec![0; num];
    let mut invp = vec![0; num];

    for i in 0..num {
        let mut vec = (0..num).collect::<Vec<usize>>();
        let mut vec2 = vec.clone();

        for l in 0..num {
            if perm2[l] == 1 {
                if 2 * (l / 2) < l {
                    nperm.push(vec[0]);
                    vec = vec[1..(num - l + 1)].to_vec();
                } else {
                    nperm.push(vec2[0]);
                    vec2 = vec2[1..(num - l + 1)].to_vec();
                }
            } else if perm2[l] == 2 {
                if 2 * (l / 2) < l {
                    nperm.push(vec[1]);
                    vec = vec.iter().enumerate().filter(|&(j, _)| j != 1).map(|(_, &v)| v).collect();
                } else {
                    nperm.push(vec2[1]);
                    vec2 = vec2.iter().enumerate().filter(|&(j, _)| j != 1).map(|(_, &v)| v).collect();
                }
            } else if perm2[l] == num - l + 1 {
                if 2 * (l / 2) < l {
                    nperm.push(vec[perm2[l] - 1]);
                    vec = vec[1..perm2[l] - 1].iter().cloned().chain(vec[0..1].iter().cloned()).collect();

                } else {
                    nperm.push(vec2[perm2[l] - 1]);
                    vec2 = vec2[1..perm2[l] - 1].iter().cloned().chain(vec2[0..1].iter().cloned()).collect();

                }
            } else {
                if 2 * (l / 2) < l {
                    nperm.push(vec[perm2[l] - 1]);
                    vec = vec[1..perm2[l] - 1]
                        .iter()
                        .map(|&x| x)  // Converte i riferimenti in valori concreti
                        .chain(vec[perm2[l]..num - l + 1].iter().map(|&x| x))  // Applica la stessa trasformazione qui
                        .collect();

                } else {
                    nperm.push(vec2[perm2[l] - 1]);
                    vec2 = vec2[1..perm2[l] - 1]
                        .iter()
                        .cloned()  // Clona ogni elemento per ottenere `usize` anziché `&usize`
                        .chain(vec2[perm2[l]..num - l + 1].iter().cloned())
                        .collect();

                }
            }
        }
        for i in 0..num {
            for j in i..num {
                if nperm[j] == i {
                    invp[i] = j - i + 1;
                    let z = nperm[j];
                    nperm[j] = nperm[i];
                    nperm[i] = z;
                }
            }
        }
    }
    invp
}

// Implementazione della funzione mapint per l'interleaving
fn mapint(ls: usize, ern: &[i32], perm2: &[usize]) -> Vec<i32> {
    let num = perm2.len();
    let mut out = vec![];

    let parse = if ls / num == 0 { 1 } else { ls / num };

    for j in 0..parse {
        let mut t2 = ern[(num * (j as usize))..(num * (j as usize + 1))].to_vec();
        if t2.iter().sum::<i32>() == t2.len() as i32 {
            out.extend(t2);
        } else {
            for l in 0..num {
                if perm2[l] == 1 {
                    if 2 * (l / 2) < l {
                        out.push(t2[0]);
                        t2 = t2[1..(1 + num - l)].to_vec();
                    } else {
                        out.push(t2[0]);
                        t2 = t2[1..(1 + num - l)].to_vec();
                    }
                } else if perm2[l] == 2 {
                    if 2 * (l / 2) < l {
                        out.push(t2[1]);
                        t2 = vec![t2[0]].into_iter()
                            .chain(t2[2..(1 + num - l)].iter().cloned())  // Clona i riferimenti per ottenere `i32`
                            .collect();

                    } else {
                        out.push(t2[1]);
                        t2 = vec![t2[0]].into_iter()
                            .chain(t2[2..(1 + num - l)].iter().cloned())  // Clona i riferimenti a `i32`
                            .collect::<Vec<i32>>();  // Specifica esplicitamente il tipo di raccolta

                    }
                } else {
                    if 2 * (l / 2) < l {
                        out.push(t2[perm2[l] - 1]);
                        t2 = t2[1..perm2[l] - 1].iter().chain(&t2[0..1]).chain(&t2[perm2[l]..(1 + num - l)]).cloned().collect();
                    } else {
                        out.push(t2[perm2[l] - 1]);
                        t2 = t2[1..perm2[l] - 1].iter().chain(&t2[0..1]).chain(&t2[perm2[l]..(1 + num - l)]).cloned().collect();
                    }
                }
            }
            out.clear();
        }
    }
    out
}

// Implementazione della funzione mapdint per il deinterleaving
fn mapdint(ls: usize, ern: &[i32], perm2: &[usize]) -> Vec<i32> {
    let num = perm2.len();
    let mut out = vec![];

    let parse = if ls / num == 0 { 1 } else { ls / num };

    for j in 0..parse {
        let mut t2 = ern[(num * (j as usize))..(num * (j as usize + 1))].to_vec();
        if t2.iter().sum::<i32>() == t2.len() as i32 {
            out.extend(t2);
        } else {
            for l in 0..num {
                if perm2[l] == 1 {
                    if 2 * (l / 2) < l {
                        out.push(t2[0]);
                        t2 = t2[1..(1 + num - l)].to_vec();
                    } else {
                        out.push(t2[0]);
                        t2 = t2[1..(1 + num - l)].to_vec();
                    }
                } else if perm2[l] == 2 {
                    if 2 * (l / 2) < l {
                        out.push(t2[1]);
                        t2 = vec![t2[0]].into_iter()
                            .chain(t2[2..(1 + num - l)].iter().cloned())  // Clona gli elementi durante la creazione dell'iteratore
                            .collect::<Vec<i32>>();  // Raccogli in `Vec<i32>`

                    } else {
                        out.push(t2[1]);
                        t2 = vec![t2[0]]
                            .into_iter()
                            .chain(t2[2..(1 + num - l)].iter().cloned())  // Usa `cloned()` per ottenere valori `i32` da `&i32`
                            .collect::<Vec<i32>>();  // Colleziona in un `Vec<i32>`

                    }
                } else {
                    if 2 * (l / 2) < l {
                        out.push(t2[perm2[l] - 1]);
                        t2 = t2[1..perm2[l] - 1].iter().chain(&t2[0..1]).chain(&t2[perm2[l]..(1 + num - l)]).cloned().collect();
                    } else {
                        out.push(t2[perm2[l] - 1]);
                        t2 = t2[1..perm2[l] - 1].iter().chain(&t2[0..1]).chain(&t2[perm2[l]..(1 + num - l)]).cloned().collect();
                    }
                }
            }
            out.clear();
        }
    }
    out
}
