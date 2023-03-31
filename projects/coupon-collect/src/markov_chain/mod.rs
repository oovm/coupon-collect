use std::iter::from_generator;
use ndarray::Array2;

use num::{BigInt, One};
use num::rational::Ratio;

pub struct MarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck
    weights: Vec<Ratio<BigInt>>,
}

#[derive(Debug)]
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
    pub fn get_transition(&self, delta: &[usize], probabilities: &[Ratio<BigInt>]) -> StateTransition {
        let mut transition = vec![0; self.count_kind()];
        let mut probability = Ratio::one();
        for (i, &d) in delta.iter().enumerate() {
            transition[i] = d;
            probability *= probabilities[i].pow(d as i32);
        }
        StateTransition {
            transition,
            probability,
        }
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
        println!("{:?}", mc.get_transition(&state, &probabilities));
    }
}