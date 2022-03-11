// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Staging Primitives.

use parity_scale_codec::{Decode, Encode};
use sp_std::{collections::btree_map::BTreeMap, prelude::*};

pub use crate::v2::*;

sp_api::decl_runtime_apis! {
	/// The API for querying the state of parachains on-chain.
	// In the staging API, this is u32::MAX.
	#[api_version(4294967295)]
  pub trait ParachainHost<H: Encode + Decode = Hash, N: Encode + Decode = BlockNumber> {
    /// Get the current validators.
		fn validators() -> Vec<ValidatorId>;

		/// Returns the validator groups and rotation info localized based on the hypothetical child
		///  of a block whose state  this is invoked on. Note that `now` in the `GroupRotationInfo`
		/// should be the successor of the number of the block.
		fn validator_groups() -> (Vec<Vec<ValidatorIndex>>, GroupRotationInfo<N>);

		/// Yields information on all availability cores as relevant to the child block.
		/// Cores are either free or occupied. Free cores can have paras assigned to them.
		fn availability_cores() -> Vec<CoreState<H, N>>;

		/// Yields the persisted validation data for the given `ParaId` along with an assumption that
		/// should be used if the para currently occupies a core.
		///
		/// Returns `None` if either the para is not registered or the assumption is `Freed`
		/// and the para already occupies a core.
		fn persisted_validation_data(para_id: Id, assumption: OccupiedCoreAssumption)
			-> Option<PersistedValidationData<H, N>>;

		/// Returns the persisted validation data for the given `ParaId` along with the corresponding
		/// validation code hash. Instead of accepting assumption about the para, matches the validation
		/// data hash against an expected one and yields `None` if they're not equal.
		fn assumed_validation_data(
			para_id: Id,
			expected_persisted_validation_data_hash: Hash,
		) -> Option<(PersistedValidationData<H, N>, ValidationCodeHash)>;

		/// Checks if the given validation outputs pass the acceptance criteria.
		fn check_validation_outputs(para_id: Id, outputs: CandidateCommitments) -> bool;

		/// Returns the session index expected at a child of the block.
		///
		/// This can be used to instantiate a `SigningContext`.
		fn session_index_for_child() -> SessionIndex;

		/// Fetch the validation code used by a para, making the given `OccupiedCoreAssumption`.
		///
		/// Returns `None` if either the para is not registered or the assumption is `Freed`
		/// and the para already occupies a core.
		fn validation_code(para_id: Id, assumption: OccupiedCoreAssumption)
			-> Option<ValidationCode>;

		/// Get the receipt of a candidate pending availability. This returns `Some` for any paras
		/// assigned to occupied cores in `availability_cores` and `None` otherwise.
		fn candidate_pending_availability(para_id: Id) -> Option<CommittedCandidateReceipt<H>>;

		/// Get a vector of events concerning candidates that occurred within a block.
		fn candidate_events() -> Vec<CandidateEvent<H>>;

		/// Get all the pending inbound messages in the downward message queue for a para.
		fn dmq_contents(
			recipient: Id,
		) -> Vec<InboundDownwardMessage<N>>;

		/// Get the contents of all channels addressed to the given recipient. Channels that have no
		/// messages in them are also included.
		fn inbound_hrmp_channels_contents(recipient: Id) -> BTreeMap<Id, Vec<InboundHrmpMessage<N>>>;

		/// Get the validation code from its hash.
		fn validation_code_by_hash(hash: ValidationCodeHash) -> Option<ValidationCode>;

		/// Scrape dispute relevant from on-chain, backing votes and resolved disputes.
		fn on_chain_votes() -> Option<ScrapedOnChainVotes<H>>;

		/***** Added in v2 *****/

		/// Get the session info for the given session, if stored.
		///
		/// NOTE: This function is only available since parachain host version 2.
		fn session_info(index: SessionIndex) -> Option<SessionInfo>;

		/// Submits a PVF pre-checking statement into the transaction pool.
		///
		/// NOTE: This function is only available since parachain host version 2.
		fn submit_pvf_check_statement(stmt: PvfCheckStatement, signature: ValidatorSignature);

		/// Returns code hashes of PVFs that require pre-checking by validators in the active set.
		///
		/// NOTE: This function is only available since parachain host version 2.
		fn pvfs_require_precheck() -> Vec<ValidationCodeHash>;

		/// Fetch the hash of the validation code used by a para, making the given `OccupiedCoreAssumption`.
		///
		/// NOTE: This function is only available since parachain host version 2.
		fn validation_code_hash(para_id: Id, assumption: OccupiedCoreAssumption)
			-> Option<ValidationCodeHash>;


		/***** Replaced in v2 *****/

		/// Old method to fetch v1 session info.
		#[changed_in(2)]
		fn session_info(index: SessionIndex) -> Option<OldV1SessionInfo>;
  }

}
