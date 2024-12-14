
pub struct TurboDecoder {
    u: Vec<i32>,
    u2: Vec<i32>,
    y1: Vec<f64>,
    y2: Vec<f64>,
    gam_sys1: Vec<f64>,
    gam_sys2: Vec<f64>,
    gam_syso2: Vec<f64>,
    gam_ry11: Vec<f64>,
    gam_ry12: Vec<f64>,
    gam_ry21: Vec<f64>,
    gam_ry22: Vec<f64>,
    p1: f64,  // Probabilità di errore
    r13: i32, // Tipo di codifica (1/2 o 1/3)
    ls: usize, // Lunghezza del messaggio
}

impl TurboDecoder{

    pub fn new(u: Vec<i32>, p1: f64, r13: i32, ls: usize) -> Self {
        Self {
            u,
            u2: vec![0; ls + 2],
            y1: vec![0.0; ls + 2],
            y2: vec![0.0; ls + 2],
            gam_sys1: vec![0.0; ls + 2],
            gam_sys2: vec![0.0; ls + 2],
            gam_syso2: vec![0.0; ls + 2],
            gam_ry11: vec![0.0; ls + 2],
            gam_ry12: vec![0.0; ls + 2],
            gam_ry21: vec![0.0; ls + 2],
            gam_ry22: vec![0.0; ls + 2],
            p1,
            r13,
            ls,
        }
    }

    pub fn decode(&mut self) -> Vec<i32> {

        // Calculating statistics for each received bit stream
        for i in 0..self.ls + 2 {
            self.gam_sys1[i] = 0.0;
            self.gam_sys2[i] = 0.0;

            // For rate 1/3, systematic bit is transmitted
            if self.r13 == 1 {
                if self.u2[i] == 1 {
                    self.gam_sys2[i] -= (1.0 - self.p1).ln() - self.p1.ln();
                } else if self.u2[i] == -1 {
                    self.gam_sys2[i] += (1.0 - self.p1).ln() - self.p1.ln();
                }
            }

            // For upper RSC (y1), using the received values
            self.gam_ry11[i] = 0.0;
            self.gam_ry12[i] = -self.y1[i] * ((1.0 - self.p1).ln() - self.p1.ln());

            // For lower RSC (y2), using the received values
            self.gam_ry21[i] = 0.0;
            self.gam_ry22[i] = -self.y2[i] * ((1.0 - self.p1).ln() - self.p1.ln());
        }

        self.gam_syso2.clone_from(&self.gam_sys2);

        ///DECODING LOOP MAP ALGORITHM  A POSTERIORI


        unimplemented!()

    }
}