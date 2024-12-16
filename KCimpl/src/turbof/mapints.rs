
/// versione mia
pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>)-> Vec<i32>{ //input trasposto da 0 per rust

    let mut out=Vec::new();

    let num=perm2.len();
    let mut parse = f64::floor(ls as f64 / num as f64) as usize;
    // let mut t2=Vec::new();
    // let mut t=Vec::new();
    if parse==0{
        parse=1;
    }

    let mut t :Vec<i32>=Vec::new();
    let mut t2:Vec<i32>=Vec::new();


    //ciclo for esterno
    for j in 1..=parse
    {
        let start_range=num*(j-1)+1; //ranges in matlab
        let end_range=num*j;
        t2=ern[start_range-1..=end_range-1].to_vec();

        if t2.iter().sum::<i32>() == t2.len() as i32
        {
            for n in 0..t2.len(){
                out.push(t2[n]);
            }
        } else {


            for l in 0..num{
                if perm2[l]==0
                {
                    if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize //l is odd
                    {
                        out.push(t2[0]);

                        let start_range=2;
                        let end_range=num-l;

                        t=t2[start_range-1..=end_range-1].to_vec(); //matlab indici

                        t2=Vec::new(); //clear operation

                    } else{ //l is even, work with t
                        out.push(t[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        t2= t[start_range - 1..=end_range - 1].to_vec();
                        t=Vec::new();

                    }
                } else if perm2[l]==1 {
                    if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize{ //l is odd

                        out.push(t2[1]);

                        t.push(t2[0]);

                        let start_range=3;
                        let end_range=1+num-l;
                        let tmp= t2[start_range - 1..=end_range - 1].to_vec();
                        t.extend(tmp);

                        t2=Vec::new();
                    } else { //l is even
                        out.push(t[1]);
                       // let mut t2=t2.to_vec();
                        let start_range=3;
                        let end_range=1+num-l;
                        t2 =Vec::new();
                        t2.push(t[0]);
                        let tmp=t[start_range-1..=end_range-1].to_vec();
                        t2.extend(tmp);
                        t=Vec::new();
                    }
                }
                else{
                    if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize{ //l is odd

                        out.push(t2[perm2[l] as usize]);


                        if perm2[l]+1> (1 + num - l) as i32 {
                            let start_range:usize=2;
                            let end_range:usize=perm2[l] as usize-1;
                            t=t2[start_range-1..=end_range-1].to_vec();
                            t.push(t2[0]);
                        }else {
                            let start_range: usize=2;
                            let end_range:usize=perm2[l] as usize-1;
                            t=Vec::new();

                            t=t2[start_range-1..=end_range-1].to_vec();

                            t.push(t2[0]);

                            let start_range=perm2[l] as usize+1;
                            let end_range=1+num-l;
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);

                        }
                        t2=Vec::new();
                    } else{

                        out.push(t[perm2[l] as usize]);

                        t2=Vec::new();
                        let start_range=2;
                       // let end_range=perm2[l] as usize-1;
                        println!(" l {:?} perm2[l] {:?}", l, perm2[l]);

                        let end_range=perm2[l] as usize;
                        t2=t[start_range-1..=end_range-1].to_vec();
                        println!("t2 {:?}", t2);
                        t2.push(t[0]);
                        println!("t2 {:?}", t2);
                        let start_range:usize=perm2[l] as usize+2;
                        let end_range=num-l;

                        let tmp=t[start_range-1..=end_range-1].to_vec();
                        t2.extend(tmp);
                        println!("t2 {:?}", t2);
                        t=Vec::new();
                    }
                }
            }
            t=Vec::new();
            t2=Vec::new();
        }
    }
    out

}
///versione AI veci32
///
// pub fn mapint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32> {
//     let mut out = Vec::new();
//     let num = perm2.len();
//     let mut parse = ls / num;
//     if parse == 0 {
//         parse = 1;
//     }
//     let mut t:Vec<i32>=Vec::new();
//
//     for j in 0..parse {
//         let start = num * j;
//         let end = num * (j + 1);
//         let t2 = ern[start..end].to_vec();
//         println!(" t2 {:?}", t2);
//         if t2.iter().sum::<i32>() == t2.len() as i32 {
//             out.extend(t2);
//         } else {
//             let mut t2 = t2;
//             for l in 0..num {
//                 if perm2[l] == 0 {
//                     if (2 * f64::floor(l as f64 / 2.0) as usize) < l {
//                         out.push(t2[0]);
//                         t2 = t2[1..(1 + num - l)].to_vec();
//                     } else {
//                         out.push(t[1]);
//                         t2 = vec![t2[0]].into_iter().chain(t2[2..(1 + num - l)].iter().cloned()).collect();
//                     }
//                 } else if perm2[l] == 1 {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[1]);
//                         t2 = vec![t2[0]].into_iter().chain(t2[2..(1 + num - l)].iter().cloned()).collect();
//                     } else {
//                         out.push(t2[1]);
//                         t2 = vec![t2[0]].into_iter().chain(t2[2..(1 + num - l)].iter().cloned()).collect();
//                     }
//                 } else {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[perm2[l] as usize]);
//                         let mut t = if perm2[l] + 1 > (1 + num - l) as i32 {
//                             t2[1..perm2[l] as usize - 1].to_vec()
//                         } else {
//                             let mut temp = t2[1..perm2[l] as usize - 1].to_vec();
//                             temp.push(t2[0]);
//                             temp.extend_from_slice(&t2[(perm2[l] + 1) as usize..(1 + num - l)]);
//                             temp
//                         };
//                         t2 = t;
//                     } else {
//                         out.push(t2[perm2[l] as usize]);
//                         let mut t = t2[1..perm2[l] as usize - 1].to_vec();
//                         t.push(t[0]);
//                         t.extend_from_slice(&t2[(perm2[l] + 1) as usize..(1 + num - l)]);
//                         t2 = t;
//                     }
//                 }
//             }
//
//             // if 2 * (num / 2) < num {
//             //     out.push(t2[3]);
//             //     out.push(t2[2]);
//             //     out.push(t2[0]);
//             //     out.push(t2[1]);
//             // } else {
//             //     out.push(t2[3]);
//             //     out.push(t2[2]);
//             //     out.push(t2[0]);
//             //     out.push(t2[1]);
//             // }
//         }
//     }
//     out
// }

pub fn invperm(perm2: &[usize]) -> Vec<i32> {
    let num = perm2.len();
    let mut vec: Vec<usize> = (1..=num).collect();
    let mut nperm: Vec<usize> = Vec::new();

    for l in 0..num {
        if perm2[l] == 1 {
            if 2 * (l / 2) < l {
                nperm.push(vec[0]); // generate output
                vec.remove(0); // resize the vector
            } else {
                nperm.push(vec[1]); // generate output
                vec.remove(1); // resize the vector
            }
        } else if perm2[l] == 2 {
            if 2 * (l / 2) < l {
                nperm.push(vec[1]); // generate output
                vec.remove(1); // resize vector
            } else {
                nperm.push(vec[1]);
                vec.remove(1);
            }
        } else if perm2[l] == num - l {
            if 2 * (l / 2) < l {
                nperm.push(vec[perm2[l] - 1]); // generate output
                vec.remove(perm2[l] - 1);
            } else {
                nperm.push(vec[perm2[l] - 1]);
                vec.remove(perm2[l] - 1);
            }
        } else {
            if 2 * (l / 2) < l {
                nperm.push(vec[perm2[l] - 1]); // generate output
                vec.remove(perm2[l] - 1);
            } else {
                nperm.push(vec[perm2[l] - 1]);
                vec.remove(perm2[l] - 1);
            }
        }
    }

    // Finding the inverse permutation:
    let mut invp: Vec<i32> = vec![0; num];
    let mut nperm = nperm.clone();

    for i in 0..num {
        for j in i..num {
            if nperm[j] == i + 1 {
                invp[i] = (j + 1) as i32;
                nperm.swap(j, i);
            }
        }
    }

    invp
}

fn mapdint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    let num = perm2.len();
    let mut parse = ls / num;
    if parse == 0 {
        parse = 1;
    }

    for j in 0..parse {
        let start = num * j;
        let end = num * (j + 1);
        let t2 = &ern[start..end];

        if t2.iter().sum::<i32>() == t2.len() as i32 {
            out.extend_from_slice(t2);
        } else {
            let mut t2 = t2.to_vec();
            for l in 0..num {
                if perm2[l] == 1 {
                    if 2 * (l / 2) < l {
                        out.push(t2[0]);
                        t2.remove(0);
                    } else {
                        out.push(t2[1]);
                        t2.remove(1);
                    }
                } else if perm2[l] == 2 {
                    if 2 * (l / 2) < l {
                        out.push(t2[1]);
                        t2.remove(1);
                        t2.remove(0);
                    } else {
                        out.push(t2[1]);
                        t2.remove(1);
                        t2.remove(0);
                    }
                } else {
                    if 2 * (l / 2) < l {
                        out.push(t2[perm2[l] as usize]);
                        let temp = t2.remove(perm2[l] as usize);
                        t2.insert(0, temp);
                    } else {
                        out.push(t2[perm2[l] as usize]);
                        let temp = t2.remove(perm2[l] as usize);
                        t2.insert(0, temp);
                    }
                }
            }

            if 2 * (num / 2) < num {
                out.push(t2[2]);
                out.push(t2[3]);
                out.push(t2[1]);
                out.push(t2[0]);
            } else {
                out.push(t2[2]);
                out.push(t2[3]);
                out.push(t2[1]);
                out.push(t2[0]);
            }
        }
    }
    out
}

pub fn mapint_f64(ls: usize, ern: Vec<f64>, perm2: Vec<usize>) -> Vec<f64> {
    // Script file for performing the forward permutations. Input
    // needed in this routine are perm2, ls, and ern. The output is out.
    //
    // The permuted soft metrics (i.e., dapp) are treated as new systematic
    // bit metrics "gamsys" for use in the next alpha and beta recursions.
    //
    // Permuting:

    let mut out: Vec<f64> = Vec::new();

    // In the actual encoder, only ls bits are permuted anyways:
    let num = perm2.len();
    let parse = if ls / num == 0 { 1 } else { ls / num }; // number of data blocks

    for j in 0..parse {
        let start_idx = num * j;
        let end_idx = num * (j + 1);
        let mut t2: Vec<f64> = ern[start_idx..end_idx].to_vec();
        let mut t:Vec<f64>=Vec::new();


        if t2.iter().sum::<f64>() == t2.len() as f64 { // checking to see if t is all ones
            out.extend(t2);
        } else {
            for l in 0..num {
                let l = l + 1; // adjusting for 1-based indexing
                let perm_idx = perm2[l-1];

                if perm_idx == 1 {
                    if 2 * (l / 2) < l { // l is odd, work with t2
                        out.push(t2[0]); // generate output
                        let mut t: Vec<f64> = t2[1..=(num-l)].to_vec(); // resize the vector
                        t2 = Vec::new();
                    } else { // l is even, work with t
                        out.push(t[0]); // generate output
                        t2 = t[1..=(num-l)].to_vec(); // resize the vector
                        t = Vec::new();
                    }
                } else if perm_idx == 2 {
                    if 2 * (l / 2) < l { // l is odd, work with t2
                        out.push(t2[1]); // generate output
                        let mut new_t = Vec::new();
                        new_t.push(t2[0]);
                        new_t.extend_from_slice(&t2[2..=(num-l)]);
                        t = new_t;
                        t2 = Vec::new();
                    } else { // l is even, work with t
                        out.push(t[1]);
                        let mut new_t2 = Vec::new();
                        new_t2.push(t[0]);
                        new_t2.extend_from_slice(&t[2..=(num-l)]);
                        t2 = new_t2;
                        t = Vec::new();
                    }
                } else {
                    if 2 * (l / 2) < l { // l is odd, work with t2
                        out.push(t2[perm_idx-1]); // generate output
                        let mut new_t = Vec::new();
                        new_t.extend_from_slice(&t2[1..perm_idx-1]);
                        new_t.push(t2[0]);
                        if perm_idx <= num-l {
                            new_t.extend_from_slice(&t2[perm_idx..=(num-l)]);
                        }
                        t = new_t;
                        t2 = Vec::new();
                    } else { // l is even, work with t
                        out.push(t[perm_idx-1]);
                        let mut new_t2 = Vec::new();
                        new_t2.extend_from_slice(&t[1..perm_idx-1]);
                        new_t2.push(t[0]);
                        if perm_idx <= num-l {
                            new_t2.extend_from_slice(&t[perm_idx..=(num-l)]);
                        }
                        t2 = new_t2;
                        t = Vec::new();
                    }
                }
            }
        }
    }

    out
}





//versione AI
// pub fn mapint(ls: usize, ern: &[usize], perm2: &[usize]) -> Vec<usize> {
//     let mut out = Vec::new();
//     let num = perm2.len();
//     let mut parse = ls / num;
//     if parse == 0 {
//         parse = 1;
//     }
//
//     for j in 0..parse {
//         let start = num * j;
//         let end = num * (j + 1);
//         let t2 = &ern[start..end];
//
//         if t2.iter().sum::<usize>() == t2.len() {
//             out.extend_from_slice(t2);
//         } else {
//             let mut t2 = t2.to_vec();
//             for l in 0..num {
//                 if perm2[l] == 1 {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[0]);
//                         t2 = t2[1..(1 + num - l)].to_vec();
//                     } else {
//                         out.push(t2[0]);
//                         let t = t2[1..(1 + num - l)].to_vec();
//                         t2 = t;
//                     }
//                 } else if perm2[l] == 2 {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[1]);
//                         let t = vec![t2[0]].into_iter().chain(t2[2..(1 + num - l)].iter().cloned()).collect::<Vec<_>>();
//                         t2 = t;
//                     } else {
//                         out.push(t2[1]);
//                         let t = vec![t2[0]].into_iter().chain(t2[2..(1 + num - l)].iter().cloned()).collect::<Vec<_>>();
//                         t2 = t;
//                     }
//                 } else {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[perm2[l]]);
//                         let mut t = if perm2[l] + 1 > 1 + num - l {
//                             t2[1..perm2[l] - 1].to_vec()
//                         } else {
//                             let mut temp = t2[1..perm2[l] - 1].to_vec();
//                             temp.extend_from_slice(&t2[perm2[l] + 1..(1 + num - l)]);
//                             temp
//                         };
//                         t2.clear();
//                     } else {
//                         out.push(t2[perm2[l]]);
//                         let mut t = t2[1..perm2[l] - 1].to_vec();
//                         t.push(t2[0]);
//                         t.extend_from_slice(&t2[perm2[l] + 1..(1 + num - l)]);
//                         t2 = t;
//                     }
//                 }
//             }
//             t2.clear();
//         }
//     }
//     out
// }

