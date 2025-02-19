
//! Autogenerated weights for `pallet_encointer_reputation_commitments`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// pallet
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_reputation_commitments
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_reputation_commitments.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_encointer_reputation_commitments`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_reputation_commitments::WeightInfo for WeightInfo<T> {
	/// Storage: EncointerReputationCommitments CurrentPurposeId (r:1 w:1)
	/// Proof: EncointerReputationCommitments CurrentPurposeId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: EncointerReputationCommitments Purposes (r:0 w:1)
	/// Proof: EncointerReputationCommitments Purposes (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	fn register_purpose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1493`
		// Minimum execution time: 21_094_000 picoseconds.
		Weight::from_parts(22_125_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EncointerReputationCommitments Purposes (r:1 w:0)
	/// Proof: EncointerReputationCommitments Purposes (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: EncointerCeremonies ParticipantReputation (r:1 w:0)
	/// Proof Skipped: EncointerCeremonies ParticipantReputation (max_values: None, max_size: None, mode: Measured)
	/// Storage: EncointerReputationCommitments Commitments (r:1 w:1)
	/// Proof: EncointerReputationCommitments Commitments (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	fn commit_reputation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329`
		//  Estimated: `3794`
		// Minimum execution time: 36_854_000 picoseconds.
		Weight::from_parts(39_205_000, 0)
			.saturating_add(Weight::from_parts(0, 3794))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
