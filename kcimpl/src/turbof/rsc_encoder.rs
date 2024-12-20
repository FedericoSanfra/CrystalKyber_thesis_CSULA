pub struct RSCEncoder{
    input: Vec<i32>,
    q: Vec<i32>,
}

impl RSCEncoder{
    pub fn new(ls: usize)->Self{

        let mut q=vec![0;ls+2];
        q.push(1);
        q.push(1); // stati inizializzazione
        Self{
            input: Vec::new(),
            q,
        }
    }

//number definisce se 0 è il upper rsc o 1 ed identifica il lower rsc, senza l'estensione del bit sistematico
    pub fn encode1(&mut self, mut input: Vec<i32>) ->(Vec<i32>, Vec<i32>) {
        let ls = input.len();
        let mut y: Vec<i32> = vec![i32::MIN;ls+2]; // Per y1 (output encoder)
        input.push(i32::MIN);
        input.push(i32::MIN); //ls+2


    // Codifica principale
        for i in 2..ls + 2 {
            self.q[i] = input[i - 2] * self.q[i - 1] * self.q[i - 2];

            y[i-2]=self.q[i - 2] * self.q[i];
        }

        // Terminazione del trellis e estensione del bit sistematico
        for i in 2..4 {
            self.q[ls+i] = (self.q[ls + i - 1] * self.q[ls + i - 2]).pow(2); // Sempre 1

            y[ls+i-2]=self.q[ls + i] * self.q[ls + i - 2];


            input[ls+i-2]=self.q[ls + i - 1] * self.q[ls + i - 2];

        }
        let sys=input.clone(); //sys input sistematico + 2, y codifica da rsc, che sarebbe u vector

        (sys, y) // Ritorna l'output codificato

    }

    pub fn encode2(&mut self, mut input: Vec<i32>) ->(Vec<i32>, Vec<i32>) {
        let ls = input.len();
        //input sarebbe up=out versione interleaved di input u
        let mut y: Vec<i32> = vec![i32::MIN;ls+2]; // Per y1 (output encoder)

        // Codifica principale
        for i in 2..ls + 2 {
            self.q[i] = input[i - 2] * self.q[i - 1] * self.q[i - 2];

            y[i-2]=self.q[i - 2] * self.q[i];
        }

        // Terminazione del trellis e estensione del bit sistematico
        for i in 2..4 {
            self.q[ls+i] = (self.q[ls + i - 1] * self.q[ls + i - 2]).pow(2); // Sempre 1

            y[ls+i-2]=self.q[ls + i] * self.q[ls + i - 2];

        }
        let sys=input.clone(); //sys input sistematico + 2, y codifica da rsc

        (sys, y) // Ritorna l'output codificato

    }
}