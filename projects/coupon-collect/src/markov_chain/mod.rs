use std::iter::from_generator;
use itertools::Itertools;
use ndarray::Array2;

pub struct MarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Starting number of cards
    start: Vec<usize>,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck
    weights: Vec<f64>,
}


impl MarkovChain {
    /// Give the weight of each option
    pub fn get_weights(&self) -> Vec<f64> {
        self.weights.clone()
    }
    /// Gives the probability of each option
    pub fn get_probabilities(&self) -> Vec<f64> {
        // normalized weights
        let sum = self.weights.iter().sum::<f64>();
        self.weights.iter().map(|x| x / sum).collect()
    }
    pub fn get_lacks(&self) -> Vec<usize> {
        self.target.iter().zip(self.start.iter()).map(|(x, y)| x.saturating_sub(*y)).collect()
    }
    pub fn transfer_matrix(&self) -> Array2<f64> {
        todo!();
        // let mut matrix = Array2::<f64>::zeros((self.weights.len(), self.weights.len()));
    }
    fn standardize(&self) -> StandardizedMarkovChain {
        let mut probabilities = self.get_probabilities();
        let target = self.get_lacks();
        StandardizedMarkovChain {
            group: self.group,
            target,
            probabilities,
        }
    }
}

pub struct StandardizedMarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck
    probabilities: Vec<f64>,
}

impl StandardizedMarkovChain {
    /// Count the number of possible states
    pub fn possible_states(&self) -> f64 {
        self.target.iter().map(|x| *x as f64 + 1.0).product()
    }
    /// (0, 0, 0) -> (0, 1, 2)
    /// - (0, 0, 0)
    /// - (0, 0, 1)
    /// - (0, 0, 2)
    /// - (0, 1, 0)
    /// - (0, 1, 1)
    /// - (0, 1, 2)
    pub fn get_states(&self) -> impl Iterator<Item=Vec<usize>> {
        let targets = self.target.clone();
        let mut state = vec![0; self.target.len()];
        from_generator(move || {
            loop {
                yield state.clone();
                let mut i = 0;
                loop {
                    state[i] += 1;
                    if state[i] > targets[i] {
                        state[i] = 0;
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        })
    }
    /// n = 3, p = a, b, c
    /// (0, 0, 0) -> (0, 0, 3) = ?
    /// (0, 0, 0) -> (0, 2, 1) = ?
    /// (0, 0, 0) -> (1, 1, 1) = ?
    pub fn get_probability(&self, delta: &[usize]) -> f64 {
        if delta.iter().sum::<usize>() != self.group {
            return 0.0;
        }
        let mut probability = 1.0;
        for state in self.get_states() {
            let mut valid = true;
            for i in 0..state.len() {
                if state[i] + delta[i] > self.target[i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                let mut p = 1.0;
                for i in 0..state.len() {
                    let target = state[i] + delta[i];
                    let index = target * self.group;
                    p *= self.probabilities[index..index + self.group].iter().product::<f64>();
                }
                probability -= p;
            }
        }
        probability
    }
}

#[test]
fn test() {
    let mc = StandardizedMarkovChain {
        group: 1,
        target: vec![0, 1, 2],
        probabilities: vec![0.3333333333333, 0.3333333333, 0.3333333333333333333],
    };
    println!("{:?}", mc.get_states().collect_vec());

    println!("{:?}", mc.get_probability(&[1, 1, 1]));
}