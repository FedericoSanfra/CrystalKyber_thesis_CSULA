use crate::turbof::rsc_encoder::RSCEncoder;
use crate::turbof::interleaver::Interleaver;
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

        let rsc_1=RSCEncoder::new();
        let rsc_2=RSCEncoder::new();

        Self{
            input,
            perm,
            rsc_1,
            rsc_2
        }
    }

    pub fn encode(&mut self) -> (Vec<i32>,Vec<i32>,Vec<i32>,Vec<i32>) {

        let ls=self.input.len();
        let (u, sys1)=self.rsc_1.encode(self.input.clone(),0); //upper rsc con estensione bit sistematico

        let out=Interleaver::mapint(ls, u.clone(), self.perm.clone());

        let (_, sys2)=self.rsc_2.encode(out.clone(),1); //lower rsc

        (u, out, sys1, sys2)

    }
}