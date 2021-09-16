// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::Network;
use snarkvm_algorithms::{
    crh::{BHPCompressedCRH, PedersenCompressedCRH},
    define_masked_merkle_tree_parameters,
};
use snarkvm_dpc::{testnet2::Testnet2Parameters, Parameters};
use snarkvm_gadgets::algorithms::crh::PedersenCompressedCRHGadget;
// use snarkvm_utilities::{FromBytes, ToBytes};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

define_masked_merkle_tree_parameters!(
    MaskedMerkleTreeParameters,
    <Testnet2 as Network>::MaskedMerkleTreeCRH,
    <Testnet2 as Network>::MASKED_TREE_DEPTH
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Testnet2;

#[rustfmt::skip]
impl Network for Testnet2 {
    /// A masked Merkle tree with depth = 2. This may change in the future.
    const MASKED_TREE_DEPTH: usize = 2;
    
    const POSW_PROOF_SIZE_IN_BYTES: usize = 771;
    
    type DPC = Testnet2Parameters;

    type BlockHeaderCRH = BHPCompressedCRH<<Self::DPC as Parameters>::ProgramProjectiveCurve, 117, 63>;
    
    type MerkleTreeCRH = BHPCompressedCRH<<Self::DPC as Parameters>::ProgramProjectiveCurve, 16, 32>;

    /// A masked Merkle tree instantiated with the masked Pedersen hash over BLS12-377.
    type MaskedMerkleTreeCRH = PedersenCompressedCRH<<<Testnet2 as Network>::DPC as Parameters>::ProgramProjectiveCurve, 4, 128>;
    type MaskedMerkleTreeCRHGadget = PedersenCompressedCRHGadget<
        <Self::DPC as Parameters>::ProgramProjectiveCurve,
        <Self::DPC as Parameters>::InnerScalarField,
        <Self::DPC as Parameters>::ProgramAffineCurveGadget,
        4,
        128
    >;
    type MaskedMerkleTreeParameters = MaskedMerkleTreeParameters;

    fn block_header_crh() -> &'static Self::BlockHeaderCRH {
        static BLOCK_HEADER_CRH: OnceCell<<Testnet2 as Network>::BlockHeaderCRH> = OnceCell::new();
        BLOCK_HEADER_CRH.get_or_init(|| Self::BlockHeaderCRH::setup("BlockHeaderCRH"))
    }

    fn merkle_tree_crh() -> &'static Self::MerkleTreeCRH {
        static MERKLE_TREE_CRH: OnceCell<<Testnet2 as Network>::MerkleTreeCRH> = OnceCell::new();
        MERKLE_TREE_CRH.get_or_init(|| Self::MerkleTreeCRH::setup("MerkleTreeCRH"))
    }

    fn masked_merkle_tree_parameters() -> &'static Self::MaskedMerkleTreeParameters {
        static MASKED_MERKLE_TREE_PARAMETERS: OnceCell<MaskedMerkleTreeParameters> = OnceCell::new();
        MASKED_MERKLE_TREE_PARAMETERS.get_or_init(|| MaskedMerkleTreeParameters::setup("MerkleTreeParameters"))
    }
}
