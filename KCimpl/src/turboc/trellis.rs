use std::cmp::PartialOrd;

pub struct Trellis {
    transition_matrix: Vec<Vec<Option<(i32, i32)>>>,
    pub(crate) past_states: Vec<(usize, usize)>,
    pub(crate) future_states: Vec<(usize, usize)>,
    pub(crate) possible_transitions: Vec<(usize, usize)>,
}

impl Trellis {
    // Funzione per calcolare il massimo tra path_metrics e branch_metrics
    pub fn butterfly<T: PartialOrd + Copy + std::ops::Add<Output = T>>(path_metrics: &[T], branch_metrics: &[T]) -> T {
        path_metrics.iter()
            .zip(branch_metrics)
            .map(|(&path, &branch)| path + branch)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    // Inizializza un nuovo oggetto Trellis
    pub fn new() -> Self {
        let transition_matrix = vec![
            vec![Some((-1, -1)), None, Some((1, 1)), None],
            vec![Some((1, -1)), None, Some((-1, 1)), None],
            vec![None, Some((-1, -1)), None, Some((1, 1))],
            vec![None, Some((1, -1)), None, Some((-1, 1))],
        ];

        let past_states = vec![(0, 1), (2, 3), (0, 1), (2, 3)];
        let future_states = vec![(0, 2), (0, 2), (1, 3), (1, 3)];

        // Creiamo tutte le transizioni possibili tra gli stati
        let possible_transitions = (0..4)
            .flat_map(|state| (0..4).map(move |next_state| (state, next_state)))
            .filter(|&(state, next_state)| transition_matrix[state][next_state].is_some())
            .collect();
        ///solo le transitions valide, che non contengono il valore None

        Trellis {
            transition_matrix,
            past_states,
            future_states,
            possible_transitions,
        }
    }

    // Ottieni il simbolo associato a una transizione, generato dai valori di rsc
    pub fn transition_to_symbols(&self, state: usize, next_state: usize) -> Option<(i32, i32)> {
        self.transition_matrix[state][next_state]
    }
}
///La struttura fissa delle matrice fa parte
/// della costruzione della macchina a stati per Trellis

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_butterfly() {
        let path_metrics = vec![1.0, 2.0, 3.0, 4.0];
        let branch_metrics = vec![0.5, 1.5, 2.5, 3.5];
        let result = Trellis::butterfly(&path_metrics, &branch_metrics);
        assert_eq!(result, 7.5); // max tra (1+0.5, 2+1.5, 3+2.5, 4+3.5)
    }

    #[test]
    fn test_transition_to_symbols() {
        let trellis = Trellis::new();
        assert_eq!(trellis.transition_to_symbols(0, 2), Some((1, 1)));
        assert_eq!(trellis.transition_to_symbols(0, 1), None);
    }

    #[test]
    fn test_possible_transitions() {
        let trellis = Trellis::new();
        assert!(trellis.possible_transitions.contains(&(0, 2)));
        assert!(!trellis.possible_transitions.contains(&(0, 1)));
    }
}
