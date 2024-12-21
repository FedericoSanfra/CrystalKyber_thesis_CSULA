use crate::turbof::rsc_encoder::RSCEncoder;
use crate::turbof::interleaver::Interleaver;
use crate::turbof::mapints::mapint;
use crate::turbof::utils::{transpositions_to_permutations, apply_permutation, reverse_permutation};
pub struct TurboEncoder{
    input: Vec<i32>,
    perm: Vec<i32>,
    rsc_1: RSCEncoder,
    rsc_2: RSCEncoder
}

impl TurboEncoder{
    pub fn new(
        input: Vec<i32>,
        perm:Vec<i32>
    )->Self{

        let rsc_1=RSCEncoder::new(input.len());
        let rsc_2=RSCEncoder::new(input.len());

        Self{
            input,
            perm,
            rsc_1,
            rsc_2
        }
    }

    pub fn encode(&mut self) -> (Vec<i32>,Vec<i32>,Vec<i32>,Vec<i32>) {

        let ls=self.input.len();
        let (u, y1)=self.rsc_1.encode1(self.input.clone()); //upper rsc con estensione bit sistematico
        //println!(" in encoder sys1 {:?}", y1);
        //println!("ls {:?} u {:?} perm {:?}", ls, u.clone(), self.perm.clone());
        let out=mapint(ls, u.clone() , self.perm.clone()); //from mapints
        // let perms=transpositions_to_permutations(self.perm.clone());
        // let out=apply_permutation(u.clone(),perms);

        //println!("out in encode {:?}", out);
        //println!("out {:?}", out);

        let (_, y2)=self.rsc_2.encode2(out.clone()); //lower rsc

        //println!("in encoder sys2 {:?}", y2); //il sistematico è u che si ripete

       // print!("y1 {:?}", y1);
        (u, out, y1, y2)

    }
}