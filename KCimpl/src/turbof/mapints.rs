///versione mia con indici matlab, correzione indici su t e t2, elemento in più in posizione 0
/// per vettore di output out
pub fn check_assign(start_range: usize, end_range: usize)->bool{
    // println!(" start range {:?} end range {:?}", start_range, end_range);
    if end_range < start_range {
        false
    } else{
        true
    }
}

pub fn invperm(mut perm2: Vec<i32>) ->Vec<i32>{

    let num=perm2.len();
    let mut vec:Vec<i32>=Vec::new();
    perm2.insert(0,-1);
   // vec.insert(0,-1);
    for i in 1..=num{
        vec.push(i as i32);
    }
    let mut vec2:Vec<i32>=Vec::new();

    let mut nperm:Vec<i32>=Vec::new();
    let mut invp:Vec<i32>=Vec::new();
    invp.insert(0,-1);
    //println!(" invp inizio {:?}", &invp);
    nperm.insert(0,-1);
    //aggiungo un elemento per poi toglierlo alla fine

    for l in 1..=num{
        if perm2[l]==1{
            if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                nperm.push(vec[0]); //decremento di uno gli indici a vec e vec2
                let start_range=2;
                let end_range=num+1-l;
                if check_assign(start_range,end_range){
                    vec2=vec[start_range-1..=end_range-1].to_vec();
                }
                vec=Vec::new();
            }else {
                nperm.push(vec2[0]);
                let start_range=2;
                let end_range=num+1-l;
                if check_assign(start_range,end_range){
                    vec=vec2[start_range-1..=end_range-1].to_vec();
                }
                vec2=Vec::new();
            }
        } else if perm2[l]==2 {
            if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                nperm.push(vec[1]);
                vec2.push(vec[0]);
                let start_range=3;
                let end_range=num+1-l;
                if check_assign(start_range,end_range){
                    let tmp=vec[start_range-1..=end_range-1].to_vec();
                    vec2.extend(tmp);
                }
                vec=Vec::new();
            } else{
                nperm.push(vec2[1]);
                vec.push(vec2[0]);
                let start_range=3;
                let end_range=num+1-l;
                if check_assign(start_range,end_range){
                    let tmp=vec2[start_range-1..=end_range-1].to_vec();
                    vec.extend(tmp);
                }
                vec2=Vec::new();
            }

        } else if perm2[l] == (num -l +1) as i32 {
            if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                nperm.push(vec[perm2[l] as usize-1]);
                let start_range=2;
                let end_range=perm2[l] as usize-1;
                if check_assign(start_range,end_range){
                    vec2=vec[start_range-1..=end_range-1].to_vec();
                }
                vec2.push(vec[0]);
                vec=Vec::new();
            } else{
                nperm.push(vec2[perm2[l] as usize-1]);
                let start_range=2;
                let end_range=perm2[l] as usize-1;
                if check_assign(start_range,end_range){
                    vec=vec2[start_range-1..=end_range-1].to_vec();
                }
                vec.push(vec2[0]);
                vec2=Vec::new();
            }
        } else {
            if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                nperm.push(vec[perm2[l] as usize-1]);
                let mut start_range=2;
                let mut end_range=perm2[l] as usize-1;
                if check_assign(start_range,end_range){
                    vec2=vec[start_range-1..=end_range-1].to_vec();
                }
                vec2.push(vec[0]);
                start_range=perm2[l] as usize+1;
                end_range=num+1-l;
                if check_assign(start_range,end_range){
                    let tmp=vec[start_range-1..=end_range-1].to_vec();
                    vec2.extend(tmp);
                }
                vec=Vec::new();
            }else{
                nperm.push(vec2[perm2[l] as usize-1]);
                let mut start_range=2;
                let mut end_range=perm2[l] as usize -1;
                if check_assign(start_range,end_range){
                    vec=vec2[start_range-1..=end_range-1].to_vec();
                }
                vec.push(vec2[0]);
                start_range=perm2[l] as usize+1;
                end_range=num+1-l;
                if check_assign(start_range,end_range){
                    let tmp=vec2[start_range-1..=end_range-1].to_vec();
                    vec.extend(tmp);
                }
                vec2=Vec::new();
            }
        }
    }
    //finding the inverse permutation

    for i in 1..=num{
        for j in i..=num{

            if nperm[j]==i as i32{
                //println!(" invp {:?}", &invp);
                //println!(" nperm {:?}", &nperm);
                invp.insert(i,(j - i + 1) as i32);
                let z=nperm[j]; //swap elements
                nperm[j]=nperm[i];
                nperm[i]=z;
            }
        }
    }
    invp.remove(0); //tolgo l'elemento in posizione 0
    invp
}

pub fn mapint(ls: usize, mut ern: Vec<i32>, mut perm2: Vec<i32>) ->Vec<i32>{
    let mut out=Vec::new();

    let num=perm2.len();
    out.insert(0,-1); //elemento mock
    perm2.insert(0,-1); //elemento mock
    ern.insert(0,-1);

    let mut t:Vec<i32>=Vec::new();
    let mut t2:Vec<i32>=Vec::new(); //devo inserire un elemento 0?
    let mut parse=f64::floor(ls as f64  / num as f64) as usize;
    //number of data blocks

    if parse==0{
        parse=1;
    }

    //////////


    for j in 1..=parse{
        t2=ern[(num*(j-1)+1)..=num*j].to_vec();
      //insert?
        if t2.iter().sum::<i32>()==t2.len() as i32{
            out.extend(t2);
        } else{

            for l in 1..=num{ //diminuisco tutti gli indici che accedono solo a t e t2
                if perm2[l]==1{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l { //l is odd, t2
                        out.push(t2[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t2=Vec::new();
                    } else { //l is even, t
                        out.push(t[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t=Vec::new();

                    }
                } else if perm2[l]==2{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l {
                        out.push(t2[1]);
                        t.push(t2[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    } else{ //l is even, work with t
                        out.push(t[1]);
                        t2.push(t[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();

                    }
                } else{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l { //l is odd, t2
                        out.push(t2[perm2[l] as usize -1]);
                        if (perm2[l]+1 )> (1 + num - l) as i32 {
                            let start_range=2;
                            let end_range=perm2[l] as usize-1;
                            if check_assign(start_range,end_range){
                                t=t2[start_range-1..=end_range-1].to_vec();
                            }
                            t.push(t2[0]);
                        } else{
                            let mut start_range=2;
                            let mut end_range=perm2[l] as usize-1;
                            //println!("check {:?}", check_assign(start_range,end_range));
                            if check_assign(start_range,end_range)  {
                                t=t2[start_range-1..=end_range-1].to_vec();
                            }
                            t.push(t2[0]);
                            start_range=perm2[l] as usize+1;
                            end_range=1+num -l;
                            if check_assign(start_range,end_range){
                                let tmp=t2[start_range-1..=end_range-1].to_vec();
                                t.extend(tmp);
                            }
                        }
                        t2=Vec::new();

                    }else{ //l is even, work with t
                        out.push(t[perm2[l] as usize -1]);
                        let mut start_range=2;
                        let mut end_range= perm2[l] as usize -1;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t2.push(t[0]);
                        start_range=perm2[l] as usize+1;
                        end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();

                    }

                }
            }
        }

    }
    out.remove(0);
    //rimuovo elemento in più aggiunto per indici
    out

}


pub fn mapint_f64(ls: usize, mut ern: Vec<f64>, mut perm2: Vec<i32>) ->Vec<f64>{
    let mut out:Vec<f64>=Vec::new();

    let num=perm2.len();
    out.insert(0,-1.0); //elemento mock
    perm2.insert(0,-1); //elemento mock
    ern.insert(0,-1.0);

    let mut t:Vec<f64>=Vec::new();
    let mut t2:Vec<f64>=Vec::new(); //devo inserire un elemento 0?
    let mut parse=f64::floor(ls as f64  / num as f64) as usize;
    //number of data blocks

    if parse==0{
        parse=1;
    }

    //////////


    for j in 1..=parse{
        t2=ern[(num*(j-1)+1)..=num*j].to_vec();
        //insert?
        if t2.iter().sum::<f64>()==t2.len() as f64{
            out.extend(t2);
        } else{

            for l in 1..=num{ //diminuisco tutti gli indici che accedono solo a t e t2
                if perm2[l]==1{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l { //l is odd, t2
                        out.push(t2[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t2=Vec::new();
                    } else { //l is even, t
                        out.push(t[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t=Vec::new();

                    }
                } else if perm2[l]==2{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l {
                        out.push(t2[1]);
                        t.push(t2[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    } else{ //l is even, work with t
                        out.push(t[1]);
                        t2.push(t[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();

                    }
                } else{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l { //l is odd, t2
                        out.push(t2[perm2[l] as usize -1]);
                        if (perm2[l]+1 )> (1 + num - l) as i32 {
                            let start_range=2;
                            let end_range=perm2[l] as usize-1;
                            if check_assign(start_range,end_range){
                                t=t2[start_range-1..=end_range-1].to_vec();
                            }
                            t.push(t2[0]);
                        } else{
                            let mut start_range=2;
                            let mut end_range=perm2[l] as usize-1;
                           // println!("check {:?}", check_assign(start_range,end_range));
                            if check_assign(start_range,end_range)  {
                                t=t2[start_range-1..=end_range-1].to_vec();
                            }
                            t.push(t2[0]);
                            start_range=perm2[l] as usize+1;
                            end_range=1+num -l;
                            if check_assign(start_range,end_range){
                                let tmp=t2[start_range-1..=end_range-1].to_vec();
                                t.extend(tmp);
                            }
                        }
                        t2=Vec::new();

                    }else{ //l is even, work with t
                        out.push(t[perm2[l] as usize -1]);
                        let mut start_range=2;
                        let mut end_range= perm2[l] as usize -1;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t2.push(t[0]);
                        start_range=perm2[l] as usize+1;
                        end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();

                    }

                }
            }
        }

    }
    out.remove(0);
    //rimuovo elemento in più aggiunto per indici
    out

}

pub fn mapdint(ls: usize, mut ern: Vec<i32>, mut perm2: Vec<i32>) ->Vec<i32>{
    let mut out=Vec::new();

    let num=perm2.len();
    out.insert(0,-1); //elemento mock
    perm2.insert(0,-1); //elemento mock
    ern.insert(0,-1);

    let mut t:Vec<i32>=Vec::new();
    let mut t2:Vec<i32>=Vec::new(); //devo inserire un elemento 0?
    let mut parse=f64::floor(ls as f64  / num as f64) as usize;
    //number of data blocks
    if parse==0{
        parse=1;
    }

    for j in 1..=parse{
        t2=ern[(num*(j-1)+1)..=(num*j)].to_vec();
        if t2.iter().sum::<i32>()==t2.len() as i32{
            out.extend(t2);
        } else{

            for l in 1..=num{
                if perm2[l]==1{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[0]); //indici decrementati di 1 su t e t2
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t2=Vec::new();
                    }else{
                        out.push(t[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t=Vec::new();
                    }
                } else if perm2[l]==2 {
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[1]);
                        t.push(t2[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    } else{ //l is even
                        out.push(t[1]);
                        t2.push(t[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();
                    }
                } else{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[perm2[l] as usize -1]);
                        let mut start_range=2;
                        let mut end_range=perm2[l] as usize-1;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t.push(t2[0]);
                        start_range=perm2[l] as usize+1;
                        end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    }else{
                        out.push(t[perm2[l] as usize -1 ]);
                        let start_range=2;
                        let end_range=perm2[l] as usize-1;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t2.push(t[0]);
                        let start_range=perm2[l] as usize+1;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();
                    }
                }
            }
            t=Vec::new();
            t2=Vec::new();
        }
    }



    out.remove(0);
    out

}

pub fn mapdint_f64(ls: usize, mut ern: Vec<f64>, mut perm2: Vec<i32>) ->Vec<f64>{
    let mut out:Vec<f64>=Vec::new();

    let num=perm2.len();
    out.insert(0,-1.0); //elemento mock
    perm2.insert(0,-1); //elemento mock
    ern.insert(0,-1.0);

    let mut t:Vec<f64>=Vec::new();
    let mut t2:Vec<f64>=Vec::new(); //devo inserire un elemento 0?
    let mut parse=f64::floor(ls as f64  / num as f64) as usize;
    //number of data blocks
    if parse==0{
        parse=1;
    }

    for j in 1..=parse{
        t2=ern[(num*(j-1)+1)..=(num*j)].to_vec();
        if t2.iter().sum::<f64>()==t2.len() as f64{
            out.extend(t2);
        } else{

            for l in 1..=num{
                if perm2[l]==1{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[0]); //indici decrementati di 1 su t e t2
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t2=Vec::new();
                    }else{
                        out.push(t[0]);
                        let start_range=2;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t=Vec::new();
                    }
                } else if perm2[l]==2 {
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[1]);
                        t.push(t2[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    } else{ //l is even
                        out.push(t[1]);
                        t2.push(t[0]);
                        let start_range=3;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();
                    }
                } else{
                    if (2 * f64::floor(l as f64 / 2.0) as usize) < l{
                        out.push(t2[perm2[l] as usize -1]);
                        let mut start_range=2;
                        let mut end_range=perm2[l] as usize-1;
                        if check_assign(start_range,end_range){
                            t=t2[start_range-1..=end_range-1].to_vec();
                        }
                        t.push(t2[0]);
                        start_range=perm2[l] as usize+1;
                        end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t2[start_range-1..=end_range-1].to_vec();
                            t.extend(tmp);
                        }
                        t2=Vec::new();
                    } else{
                        out.push(t[perm2[l] as usize -1 ]);
                        let start_range=2;
                        let end_range=perm2[l] as usize-1;
                        if check_assign(start_range,end_range){
                            t2=t[start_range-1..=end_range-1].to_vec();
                        }
                        t2.push(t[0]);
                        let start_range=perm2[l] as usize+1;
                        let end_range=1+num-l;
                        if check_assign(start_range,end_range){
                            let tmp=t[start_range-1..=end_range-1].to_vec();
                            t2.extend(tmp);
                        }
                        t=Vec::new();
                    }
                }
            }
            t=Vec::new();
            t2=Vec::new();
        }
    }



    out.remove(0);
    out

}
// versione mia
// pub fn mapint_old(ls: usize, ern: Vec<i32>, perm2: Vec<i32>)-> Vec<i32>{ //input trasposto da 0 per rust
//
//     let mut out=Vec::new();
//
//     let num=perm2.len();
//     let mut parse = f64::floor(ls as f64 / num as f64) as usize;
//     // let mut t2=Vec::new();
//     // let mut t=Vec::new();
//     if parse==0{
//         parse=1;
//     }
//
//     let mut t :Vec<i32>=Vec::new();
//     let mut t2:Vec<i32>=Vec::new();
//
//
//     //ciclo for esterno
//     for j in 1..=parse
//     {
//         let start_range=num*(j-1)+1; //ranges in matlab
//         let end_range=num*j;
//         t2=ern[start_range-1..=end_range-1].to_vec();
//
//         if t2.iter().sum::<i32>() == t2.len() as i32
//         {
//             for n in 0..t2.len(){
//                 out.push(t2[n]);
//             }
//         } else {
//
//
//             for l in 0..num{
//                 if perm2[l]==0
//                 {
//                     if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize //l is odd
//                     {
//                         out.push(t2[0]);
//
//                         let start_range=2;
//                         let end_range=num-l;
//                         if check_assign(start_range, end_range){
//                             t=t2[start_range-1..=end_range-1].to_vec(); //matlab indici
//                         }
//
//
//                         t2=Vec::new(); //clear operation
//
//                     } else{ //l is even, work with t
//                         out.push(t[0]);
//                         let start_range=2;
//                         let end_range=num-l;
//                         if check_assign(start_range,end_range){
//                             t2= t[start_range - 1..=end_range - 1].to_vec();
//                         }
//                         t=Vec::new();
//
//                     }
//                 } else if perm2[l]==1 {
//                     if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize{ //l is odd
//
//                         out.push(t2[1]);
//
//                         t.push(t2[0]);
//
//                         let start_range=3;
//                         let end_range=1+num-l;
//                         if check_assign(start_range, end_range){
//                             let tmp= t2[start_range - 1..=end_range - 1].to_vec();
//                             t.extend(tmp);
//                         }
//
//
//                         t2=Vec::new();
//                     } else { //l is even
//                         out.push(t[1]);
//                        // let mut t2=t2.to_vec();
//                         let start_range=3;
//                         let end_range=num-l;
//                         t2 =Vec::new();
//                         t2.push(t[0]);
//                         //if end_range>
//                         if check_assign(start_range,end_range){
//                             let tmp=t[start_range-1..=end_range-1].to_vec();
//                             t2.extend(tmp);
//                         }
//
//                         t=Vec::new();
//                     }
//                 }
//                 else{
//                     if ( 2 * f64::floor((l as f64+ 1.0 ) / 2.0) as usize )  < l +1 as usize{ //l is odd
//
//                         out.push(t2[perm2[l] as usize]);
//
//
//                         if perm2[l]+1> ( num - l-1) as i32 {
//                             let start_range:usize=2;
//                             let end_range:usize=perm2[l] as usize;
//                             t=t2[start_range-1..=end_range-1].to_vec();
//                             t.push(t2[0]);
//                         }else {
//                             let start_range: usize=2;
//                             let end_range:usize=perm2[l] as usize-1;
//                             t=Vec::new();
//
//                             t=t2[start_range-1..=end_range-1].to_vec();
//
//                             t.push(t2[0]);
//
//                             let start_range=perm2[l] as usize+1;
//                             let end_range=1+num-l;
//                             if check_assign(start_range,end_range){
//                                 let tmp=t2[start_range-1..=end_range-1].to_vec();
//                                 t.extend(tmp);
//                             }
//
//
//                         }
//                         t2=Vec::new();
//                     } else{
//
//                         out.push(t[perm2[l] as usize]);
//
//                         t2=Vec::new();
//                         let start_range=2;
//                        // let end_range=perm2[l] as usize-1;
//                         println!(" l {:?} perm2[l] {:?}", l, perm2[l]);
//
//                         let end_range=perm2[l] as usize;
//                         if check_assign(start_range,end_range){
//                             t2=t[start_range-1..=end_range-1].to_vec();
//                         }
//
//                         println!("t2 {:?}", t2);
//                         t2.push(t[0]);
//                         println!("t2 {:?}", t2);
//
//                         let start_range:usize=perm2[l] as usize+1;
//                         let end_range=num-l;
//                         if check_assign(start_range,end_range){
//                             let tmp=t[start_range..=end_range-1].to_vec();
//                             t2.extend(tmp);
//                         }
//
//                         println!("t2 {:?}", t2);
//                         t=Vec::new();
//                     }
//                 }
//             }
//             t=Vec::new();
//             t2=Vec::new();
//         }
//     }
//     out
//
// }


// versione AI veci32
//
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


// fn mapdint(ls: usize, ern: Vec<i32>, perm2: Vec<i32>) -> Vec<i32> {
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
//         if t2.iter().sum::<i32>() == t2.len() as i32 {
//             out.extend_from_slice(t2);
//         } else {
//             let mut t2 = t2.to_vec();
//             for l in 0..num {
//                 if perm2[l] == 1 {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[0]);
//                         t2.remove(0);
//                     } else {
//                         out.push(t2[1]);
//                         t2.remove(1);
//                     }
//                 } else if perm2[l] == 2 {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[1]);
//                         t2.remove(1);
//                         t2.remove(0);
//                     } else {
//                         out.push(t2[1]);
//                         t2.remove(1);
//                         t2.remove(0);
//                     }
//                 } else {
//                     if 2 * (l / 2) < l {
//                         out.push(t2[perm2[l] as usize]);
//                         let temp = t2.remove(perm2[l] as usize);
//                         t2.insert(0, temp);
//                     } else {
//                         out.push(t2[perm2[l] as usize]);
//                         let temp = t2.remove(perm2[l] as usize);
//                         t2.insert(0, temp);
//                     }
//                 }
//             }
//
//             if 2 * (num / 2) < num {
//                 out.push(t2[2]);
//                 out.push(t2[3]);
//                 out.push(t2[1]);
//                 out.push(t2[0]);
//             } else {
//                 out.push(t2[2]);
//                 out.push(t2[3]);
//                 out.push(t2[1]);
//                 out.push(t2[0]);
//             }
//         }
//     }
//     out
// }





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

