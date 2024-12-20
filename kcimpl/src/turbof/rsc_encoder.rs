pub struct RSCEncoder{
    input: Vec<i32>,
    q: Vec<i32>,
}

impl RSCEncoder{
    pub fn new()->Self{

        let mut q=vec![1,1]; // stati inizializzazione
        Self{
            input: Vec::new(),
            q,
        }
    }

//number definisce se 0 è il upper rsc o 1 ed identifica il lower rsc, senza l'estensione del bit sistematico
    pub fn encode(&mut self, mut input: Vec<i32>, number: usize) ->(Vec<i32>, Vec<i32>) {
        let ls = input.len();
        let mut y: Vec<i32> = Vec::with_capacity(ls + 2); // Per y1 (output encoder)

        // Codifica principale
        for i in 2..ls + 2 {
            let qi = input[i - 2] * self.q[i - 1] * self.q[i - 2];
            self.q.push(qi);
            y.push(self.q[i - 2] * self.q[i]);
        }

        // Terminazione del trellis e estensione del bit sistematico
        for i in 2..4 {
            let qi = (self.q[ls + i - 1] * self.q[ls + i - 2]).pow(2); // Sempre 1
            self.q.push(qi);
            y.push(self.q[ls + i] * self.q[ls + i - 2]);

            if(number==0){
                input.push(self.q[ls + i - 1] * self.q[ls + i - 2]);
            }
        }
        let sys=input.clone(); //sys input sistematico + 2, y codifica da rsc

        (sys, y) // Ritorna l'output codificato

    }
}