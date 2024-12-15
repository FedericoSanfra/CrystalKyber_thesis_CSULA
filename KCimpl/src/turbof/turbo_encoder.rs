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
        let (u, y1)=self.rsc_1.encode(self.input.clone(),0); //upper rsc con estensione bit sistematico
        //println!(" in encoder sys1 {:?}", y1);
        //println!("ls {:?} u {:?} perm {:?}", ls, u.clone(), self.perm.clone());
        let out=Interleaver::mapint(ls, u.clone() , self.perm.clone());
        //println!("out {:?}", out);

        let (_, y2)=self.rsc_2.encode(out.clone(),1); //lower rsc

        //println!("in encoder sys2 {:?}", y2); //il sistematico è u che si ripete

        (u, out, y1, y2)

    }
}