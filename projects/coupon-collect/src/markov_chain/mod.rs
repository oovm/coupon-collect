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
}