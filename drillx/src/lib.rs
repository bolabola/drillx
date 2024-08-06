mod operator2;
mod utils;

#[cfg(feature = "gpu")]
pub mod gpu;
pub mod noise;

#[cfg(feature = "benchmark")]
use std::time::Instant;

// use crate::operator::Operator;
use crate::operator2::Operator2;
pub use crate::utils::*;

// TODO Debug feature flag for print statements

#[cfg(not(feature = "benchmark"))]
pub fn hash(challenge: &[u8; 32], nonce: &[u8; 8]) -> [u8; 32] {
    let digest = Operator2::new(challenge, nonce).drill();
    solana_program::keccak::hashv(&[digest.as_slice()]).0
}

#[cfg(feature = "benchmark")]
pub fn hash(challenge: &[u8; 32], nonce: &[u8; 8]) -> [u8; 32] {
    // The drill part (non-parallelizable digest)
    println!("Nonce {}", u64::from_le_bytes(*nonce));
    let timer = Instant::now();
    let digest = Operator2::new(challenge, nonce).drill();
    println!("drill in {} nanos", timer.elapsed().as_nanos());

    // The hash part (keccak proof)
    let timer = Instant::now();
    let x = solana_program::keccak::hashv(&[digest.as_slice()]).0;
    println!("hash in {} nanos\n", timer.elapsed().as_nanos());
    x
}
/// A drillx solution which can be efficiently validated on-chain
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Solution {
    pub d: [u8; 16], // digest
    pub n: [u8; 8],  // nonce
}

impl Solution {
    /// Builds a new verifiable solution from a hash and nonce
    pub fn new(digest: [u8; 16], nonce: [u8; 8]) -> Solution {
        Solution {
            d: digest,
            n: nonce,
        }
    }

    /// Returns true if the solution is valid
    pub fn is_valid(&self, challenge: &[u8; 32]) -> bool {
        is_valid_digest(challenge, &self.n, &self.d)
    }

    /// Calculates the result hash for a given solution
    pub fn to_hash(&self) -> Hash {
        let mut d = self.d;
        Hash {
            d: self.d,
            h: hashv(&mut d, &self.n),
        }
    }
}
