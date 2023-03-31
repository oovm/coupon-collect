use std::iter::from_generator;
use std::ops::Mul;
use ndarray::Array2;

use num::{BigInt};
use num::rational::Ratio;

pub struct MarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck
    weights: Vec<Ratio<BigInt>>,
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    transition: Vec<usize>,
    probability: Ratio<BigInt>,
}


impl MarkovChain {
    /// Count kinds of cards
    pub fn count_kind(&self) -> usize {
        self.weights.len()
    }
    /// Count cards in a deck
    pub fn count_deck(&self) -> usize {
        self.group
    }
    /// Give the weight of each option
    pub fn get_weights(&self) -> Vec<Ratio<BigInt>> {
        self.weights.clone()
    }
    /// Gives the probability of each option
    pub fn get_probabilities(&self) -> Vec<Ratio<BigInt>> {
        // normalized weights
        let sum = self.weights.iter().sum::<Ratio<BigInt>>();
        self.weights.iter().map(|x| x / sum.clone()).collect()
    }
    pub fn transfer_matrix(&self) -> Array2<Ratio<BigInt>> {
        todo!();
        // let mut matrix = Array2::<f64>::zeros((self.weights.len(), self.weights.len()));
    }
    pub fn get_transition_states(&self) -> impl Iterator<Item=Vec<usize>> {
        let kind = self.count_kind();
        let deck = self.count_deck();
        let mut state = vec![0; kind];
        state[0] = deck;
        // eg. deck = 3, kind = 2
        // (3, 0), (2, 1), (1, 2), (0, 3)
        from_generator(move || {
            yield state.clone();
            loop {
                let mut next = state.clone();
                let mut i = 0;
                while i < kind - 1 {
                    if next[i] > 0 {
                        next[i] -= 1;
                        next[i + 1] += 1;
                        break;
                    }
                    i += 1;
                }
                if i == kind - 1 {
                    break;
                }
                state = next;
                yield state.clone();
            }
        })
    }
    pub fn get_frequency(&self, delta: &[usize]) -> Ratio<BigInt> {
        assert_eq!(delta.iter().sum::<usize>(), self.count_deck(), "delta must same as deck");
        // eg. deck = 3, kind = 2
        // (3, 0) => 1
        // (2, 1) => 3
        // (1, 2) => 3
        // (0, 3) => 1
        let mut frequency = Ratio::from_integer(BigInt::from(1));
        let mut count = 0;
        for i in 0..self.count_kind() {
            let mut j = 0;
            while j < delta[i] {
                frequency *= Ratio::from_integer(BigInt::from(count + 1));
                frequency /= Ratio::from_integer(BigInt::from(j + 1));
                j += 1;
                count += 1;
            }
        }
        frequency
    }

    pub fn get_transitions(&self) -> Vec<StateTransition> {
        let mut transitions = Vec::new();
        let probabilities = self.get_probabilities();
        for delta in self.get_transition_states() {
            let frequency = self.get_frequency(&delta);
            let probability = probabilities.iter()
                .zip(delta.iter())
                .map(|(p, d)| p.mul(BigInt::from(*d)))
                .fold(Ratio::from_integer(BigInt::from(1)), |acc, x| acc * x);
            transitions.push(StateTransition {
                transition: delta,
                probability: frequency * probability,
            });
        }
        transitions
    }
}


impl MarkovChain {
    pub fn new(group: usize) -> Self {
        Self {
            group,
            target: vec![],
            weights: vec![],
        }
    }
    pub fn define_cards<N>(&mut self, needs: usize, weight: N)
        where
            N: Into<Ratio<BigInt>>,
    {
        self.target.push(needs);
        self.weights.push(weight.into());
    }
}


#[test]
fn test() {
    let mut mc = MarkovChain::new(3);
    mc.define_cards(1, BigInt::from(1));
    mc.define_cards(1, BigInt::from(1));
    let probabilities = mc.get_probabilities();
    println!("{:?}", probabilities);
    for state in mc.get_transition_states() {
        println!("{:?}", mc.get_frequency(&state));
    }
}