use rand::Rng;
//p1 error probability del canale bsc
pub fn generate_noise_vector(ls: usize, p1: f64) -> Vec<i32> {
    // Creazione di un generatore di numeri casuali
    let mut rng = rand::thread_rng();

    // La lunghezza del vettore di rumore è 3*(ls + 2)
    let noise_length = 3 * (ls + 2);

    // Inizializza il vettore del rumore
    let mut n: Vec<i32> = Vec::with_capacity(noise_length);

    // Genera il vettore del rumore
    for _ in 0..noise_length {
        let rand_num: f64 = rng.gen(); // Numero casuale tra 0 e 1
        if rand_num > p1 {
            n.push(1);
        } else {
            n.push(-1);
        }
    }

    n
}
