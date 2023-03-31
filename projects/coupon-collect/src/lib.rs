#![feature(generators)]
#![feature(iter_from_generator)]

mod errors;

pub use errors::{Error, Result};

mod markov_chain;

pub use crate::markov_chain::MarkovChain;