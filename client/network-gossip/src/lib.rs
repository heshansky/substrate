// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

pub use self::bridge::GossipEngine;
// TODO: remove
pub use self::state_machine::*;

use network::{specialization::NetworkSpecialization, Event, ExHashT, NetworkService, PeerId};
use sr_primitives::{traits::{Block as BlockT, NumberFor}, ConsensusEngineId};
use std::sync::Arc;

mod bridge;
mod state_machine;

/// Abstraction over a network.
pub trait Network<B: BlockT> {
	/// Returns a stream of events representing what happens on the network.
	fn event_stream(&self) -> Box<dyn futures01::Stream<Item = Event, Error = ()> + Send>;

	/// Adjust the reputation of a node.
	fn report_peer(&self, peer_id: PeerId, reputation: i32);

	/// Force-disconnect a peer.
	fn disconnect_peer(&self, who: PeerId);

	/// Send a notification to a peer.
	fn write_notification(&self, who: PeerId, engine_id: ConsensusEngineId, message: Vec<u8>);

	/// Registers a notifications protocol.
	///
	/// See the documentation of [`NetworkService:register_notifications_protocol`] for more information.
	fn register_notifications_protocol(
		&self,
		engine_id: ConsensusEngineId
	);

	/// Notify everyone we're connected to that we have the given block.
	///
	/// Note: this method isn't strictly related to gossiping and should eventually be moved
	/// somewhere else.
	fn announce(&self, block: B::Hash, associated_data: Vec<u8>);

	/// Notifies the sync service to try and sync the given block from the given
	/// peers.
	///
	/// If the given vector of peers is empty then the underlying implementation
	/// should make a best effort to fetch the block from any peers it is
	/// connected to (NOTE: this assumption will change in the future #3629).
	///
	/// Note: this method isn't strictly related to gossiping and should eventually be moved
	/// somewhere else.
	fn set_sync_fork_request(&self, peers: Vec<PeerId>, hash: B::Hash, number: NumberFor<B>);
}

impl<B: BlockT, S: NetworkSpecialization<B>, H: ExHashT> Network<B> for Arc<NetworkService<B, S, H>> {
	fn event_stream(&self) -> Box<dyn futures01::Stream<Item = Event, Error = ()> + Send> {
		Box::new(NetworkService::event_stream(self))
	}

	fn report_peer(&self, peer_id: PeerId, reputation: i32) {
		NetworkService::report_peer(self, peer_id, reputation);
	}

	fn disconnect_peer(&self, who: PeerId) {
		NetworkService::disconnect_peer(self, who)
	}

	fn write_notification(&self, who: PeerId, engine_id: ConsensusEngineId, message: Vec<u8>) {
		NetworkService::write_notification(self, who, engine_id, message)
	}

	fn register_notifications_protocol(
		&self,
		engine_id: ConsensusEngineId,
	) {
		NetworkService::register_notifications_protocol(self, engine_id)
	}

	fn announce(&self, block: B::Hash, associated_data: Vec<u8>) {
		NetworkService::announce_block(self, block, associated_data)
	}

	fn set_sync_fork_request(&self, peers: Vec<network::PeerId>, hash: B::Hash, number: NumberFor<B>) {
		NetworkService::set_sync_fork_request(self, peers, hash, number)
	}
}