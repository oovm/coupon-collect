use std::iter::from_generator;
use itertools::Itertools;
use ndarray::Array2;
use num::{BigInt, One, Zero};
use num::rational::Ratio;

pub struct MarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck
    weights: Vec<Ratio<BigInt>>,
}


impl MarkovChain {
    pub fn new(group: usize) -> Self {
        Self {
            group,
            target: vec![],
            weights: vec![],
        }
    }
    pub fn define_cards(&mut self, needs: usize, weight: Ratio<BigInt>) {
        self.target.push(needs);
        self.weights.push(weight);
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
    // pub fn get_states(&self) -> impl Iterator<Item=Vec<usize>> {
    //    unimplemented!()
    // }
    pub fn transfer_matrix(&self) -> Array2<Ratio<BigInt>> {
        todo!();
        // let mut matrix = Array2::<f64>::zeros((self.weights.len(), self.weights.len()));
    }
    fn standardize(&self) -> StandardizedMarkovChain {
        let mut probabilities = self.get_probabilities();
        StandardizedMarkovChain {
            group: self.group,
            target: self.target.clone(),
            probabilities,
        }
    }
}

#[derive(Debug)]
pub struct StateTransition {
    transition: Vec<usize>,
    probability: Ratio<BigInt>,
}

pub struct StandardizedMarkovChain {
    /// Number of cards per pack
    group: usize,
    /// Target number of cards
    target: Vec<usize>,
    /// Number of cards in the deck, sum must 1
    probabilities: Vec<Ratio<BigInt>>,
}

impl StandardizedMarkovChain {
    pub fn get_probability(&self, delta: &[usize]) -> Ratio<BigInt> {
        let mut probability = Ratio::one();
        for (i, &d) in delta.iter().enumerate() {
            probability *= self.probabilities[i].pow(d as i32);
        }
        probability
    }

    pub fn get_states(&self) -> impl Iterator<Item=Vec<usize>> {
        // kinds of all cards
        let items = self.target.len();
        // cards in each pack
        let group = self.group;
        // eg, group = 3, items = 3
        // 0 0 0
        // 1,1,1
        // 0,1,2 | 0,2,1 | 1,0,2 | 1,2,0 | 2,0,1 | 2,1,0
        // 0,0,3 | 0,3,0 | 3,0,0
        for i in 0..=group {
            let mut state = vec![0; items];
            state[0] = i;
            for j in 0..=(group - i) {
                state[1] = j;
                state[2] = group - i - j;
                yield state.clone();
            }
        }
    }
    pub fn get_transition(&self) -> impl Iterator<Item=StateTransition> + '_ {
        todo!()
    }
}


#[test]
fn test() {
    let f13 = Ratio::new(BigInt::from(1), BigInt::from(3));
    let mc = StandardizedMarkovChain {
        group: 3,
        target: vec![0, 1, 2],
        probabilities: vec![f13.clone(), f13.clone(), f13.clone()],
    };
    for state in mc.get_states() {
        println!("{:?}", state);
    }
    for state in mc.get_transition() {
        println!("{:?}", state);
    }
}