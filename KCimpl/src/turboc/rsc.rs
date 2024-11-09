use std::collections::VecDeque;

#[derive(Clone)]
pub struct RSC {
    pub(crate) registers: VecDeque<u8>,
}

impl RSC {
    /// Crea una nuova istanza di RSC Encoder e resetta i registri
    pub fn new() -> Self {
        let mut rsc = RSC {
            registers: VecDeque::from(vec![0, 0]),
        };
        rsc.reset(); //per chiarezza
        rsc
    }

    /// Reset dei registri a 0
    pub fn reset(&mut self) {
        self.registers = VecDeque::from(vec![0, 0]);
    }

    /// Riceve un bit di input e restituisce il bit di parità calcolato
    pub fn push(&mut self, value: u8) -> u8 {
        // Calcolo del bit di parità con XOR tra input e ultimo registro
        let result = value ^ self.registers[1];
        self.registers.rotate_right(1); // Sposta i registri a destra
        self.registers[0] = result; // Aggiorna il primo registro col valore
        result
    }

    /// Esegue il processo di terminazione per scaricare i registri
    pub fn terminate(&mut self) -> u8 {
        let result = self.registers[1]; // Ottieni il valore dell'ultimo registro
        self.registers.rotate_right(1); // Sposta i registri a destra
        self.registers[0] = 0; // Resetta il primo registro a 0
        result
    }

    /// Codifica un intero vettore di bit di input, restituendo il risultato e la parte sistematica
    pub fn execute(&mut self, vector: Vec<usize>) -> (Vec<u8>, Vec<usize>) {
        let mut result = Vec::with_capacity(vector.len() + self.registers.len());

        // Esegue `push` su ciascun bit del vettore e memorizza i bit di parità
        for v in vector.clone() {
            result.push(self.push(v as u8));
        }

        // Esegue `terminate` per scaricare i registri
        for _ in 0..self.registers.len() {
            result.push(self.terminate());
        }

        // Costruisce il vettore "sistematico" concatenando i bit originali con i risultati dei registri
        let systematic: Vec<usize> = vector.clone()
            .iter()
            .copied()
            .chain(result[vector.len()..].iter().map(|&x| x as usize ))
            .collect();

        (result, systematic)
    }
}

// Esempio di utilizzo e test
// fn main() {
//     let mut rsc = RSC::new();
//     let input_vector = vec![1, 1, 0, 1];
//
//     // Esecuzione dell'encoder
//     let (encoded, systematic) = rsc.execute(&input_vector);
//
//     println!("Input: {:?}", input_vector);
//     println!("Encoded (with parity): {:?}", encoded);
//     println!("Systematic Output: {:?}", systematic);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        let mut rsc = RSC::new();
        rsc.push(1);
        rsc.reset();
        assert_eq!(rsc.registers, VecDeque::from(vec![0, 0]));
    }

    #[test]
    fn test_push() {
        let mut rsc = RSC::new();
        let parity = rsc.push(1);
        assert_eq!(parity, 1);
        assert_eq!(rsc.registers, VecDeque::from(vec![1, 0]));
    }

    #[test]
    fn test_terminate() {
        let mut rsc = RSC::new();
        rsc.push(1);
        let termination = rsc.terminate();
        assert_eq!(termination, 0);
        assert_eq!(rsc.registers, VecDeque::from(vec![0, 1]));
    }

    #[test]
    fn test_execute() {
        let mut rsc = RSC::new();
        let input_vector:Vec<usize> = vec![1, 1, 0, 1];
        let (encoded, systematic) = rsc.execute(input_vector.clone());

        assert_eq!(encoded.len(), input_vector.clone().len() + 2); // 2 è la dimensione dei registri
        assert_eq!(systematic.len(), input_vector.clone().len() + 2);
    }
}
