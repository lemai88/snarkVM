// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod bytes;
mod serialize;

use crate::helpers::Data;
use console::prelude::*;
use synthesizer::Transaction;
use synthesizer_coinbase::ProverSolution;

use ::bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Transmission<N: Network> {
    /// A ratification.
    Ratification,
    /// A prover solution.
    /// Attention: Observe that the solution is encapsulated in `Data`, and thus possibly unchecked.
    Solution(Data<ProverSolution<N>>),
    /// A transaction.
    /// Attention: Observe that the transaction is encapsulated in `Data`, and thus possibly unchecked.
    Transaction(Data<Transaction<N>>),
}

impl<N: Network> From<ProverSolution<N>> for Transmission<N> {
    /// Converts the prover solution into an transmission.
    fn from(solution: ProverSolution<N>) -> Self {
        Self::Solution(Data::Object(solution))
    }
}

impl<N: Network> From<Transaction<N>> for Transmission<N> {
    /// Converts the transaction into an transmission.
    fn from(transaction: Transaction<N>) -> Self {
        Self::Transaction(Data::Object(transaction))
    }
}

impl<N: Network> From<Data<ProverSolution<N>>> for Transmission<N> {
    /// Converts the prover solution into an transmission.
    fn from(solution: Data<ProverSolution<N>>) -> Self {
        Self::Solution(solution)
    }
}

impl<N: Network> From<Data<Transaction<N>>> for Transmission<N> {
    /// Converts the transaction into an transmission.
    fn from(transaction: Data<Transaction<N>>) -> Self {
        Self::Transaction(transaction)
    }
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;
    use console::{network::Testnet3, prelude::TestRng};

    use ::bytes::Bytes;
    use rand::Rng;

    type CurrentNetwork = Testnet3;

    /// Returns a list of sample transmissions, sampled at random.
    pub(crate) fn sample_transmissions(rng: &mut TestRng) -> Vec<Transmission<CurrentNetwork>> {
        // Initialize a sample vector.
        let mut sample = Vec::with_capacity(10);
        // Append sample solutions.
        for _ in 0..5 {
            // Sample random fake solution bytes.
            let solution = Data::Buffer(Bytes::from((0..1024).map(|_| rng.gen::<u8>()).collect::<Vec<_>>()));
            // Append the solution.
            sample.push(Transmission::Solution(solution));
        }
        // Append sample transactions.
        for _ in 0..5 {
            // Sample random fake transaction bytes.
            let transaction = Data::Buffer(Bytes::from((0..1024).map(|_| rng.gen::<u8>()).collect::<Vec<_>>()));
            // Append the transaction.
            sample.push(Transmission::Transaction(transaction));
        }
        // Return the sample vector.
        sample
    }
}
