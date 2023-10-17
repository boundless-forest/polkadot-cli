use super::RpcError;
use crate::rpc::errors::MetadataError;
use codec::Compact;
use scale_decode::DecodeAsType;
use sp_core::{Decode, Encode};
use std::sync::Arc;
use subxt_metadata::{Metadata, PalletMetadata};

/// A collection of events obtained from a block, bundled with the necessary
/// information needed to decode and iterate over them.
pub struct Events<Hash> {
	metadata: Metadata,
	block_hash: Hash,
	// Note; raw event bytes are prefixed with a Compact<u32> containing
	// the number of events to be decoded. The start_idx reflects that, so
	// that we can skip over those bytes when decoding them
	event_bytes: Arc<[u8]>,
	start_idx: usize,
	num_events: u32,
}

// Ignore the Metadata when debug-logging events; it's big and distracting.
impl<Hash: std::fmt::Debug> std::fmt::Debug for Events<Hash> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Events")
			.field("block_hash", &self.block_hash)
			.field("event_bytes", &self.event_bytes)
			.field("start_idx", &self.start_idx)
			.field("num_events", &self.num_events)
			.finish()
	}
}

impl<Hash: Decode> Events<Hash> {
	pub(crate) fn new(metadata: Metadata, block_hash: Hash, event_bytes: Vec<u8>) -> Self {
		// event_bytes is a SCALE encoded vector of events. So, pluck the
		// compact encoded length from the front, leaving the remaining bytes
		// for our iterating to decode.
		//
		// Note: if we get no bytes back, avoid an error reading vec length
		// and default to 0 events.
		let cursor = &mut &*event_bytes;
		let num_events = <Compact<u32>>::decode(cursor).unwrap_or(Compact(0)).0;

		// Start decoding after the compact encoded bytes.
		let start_idx = event_bytes.len() - cursor.len();

		Self { metadata, block_hash, event_bytes: event_bytes.into(), start_idx, num_events }
	}

	/// The number of events.
	pub fn len(&self) -> u32 {
		self.num_events
	}

	/// Are there no events in this block?
	// Note: mainly here to satisfy clippy.
	pub fn is_empty(&self) -> bool {
		self.num_events == 0
	}

	/// Return the block hash that these events are from.
	// pub fn block_hash(&self) -> Hash {
	// 	self.block_hash
	// }

	/// Iterate over all of the events, using metadata to dynamically
	/// decode them as we go, and returning the raw bytes and other associated
	/// details. If an error occurs, all subsequent iterations return `None`.
	// Dev note: The returned iterator is 'static + Send so that we can box it up and make
	// use of it with our `FilterEvents` stuff.
	pub fn iter(
		&self,
	) -> impl Iterator<Item = Result<EventDetails<Hash>, RpcError>> + Send + Sync + 'static {
		// The event bytes ignoring the compact encoded length on the front:
		let event_bytes = self.event_bytes.clone();
		let metadata = self.metadata.clone();
		let num_events = self.num_events;

		let mut pos = self.start_idx;
		let mut index = 0;
		std::iter::from_fn(move || {
			if event_bytes.len() <= pos || num_events == index {
				None
			} else {
				match EventDetails::decode_from(metadata.clone(), event_bytes.clone(), pos, index) {
					Ok(event_details) => {
						// Skip over decoded bytes in next iteration:
						pos += event_details.bytes().len();
						// Increment the index:
						index += 1;
						// Return the event details:
						Some(Ok(event_details))
					},
					Err(e) => {
						// By setting the position to the "end" of the event bytes,
						// the cursor len will become 0 and the iterator will return `None`
						// from now on:
						pos = event_bytes.len();
						Some(Err(e))
					},
				}
			}
		})
	}

	// /// Iterate through the events using metadata to dynamically decode and skip
	// /// them, and return only those which should decode to the provided `Ev` type.
	// /// If an error occurs, all subsequent iterations return `None`.
	// pub fn find<Ev: StaticEvent>(&self) -> impl Iterator<Item = Result<Ev, Error>> + '_ {
	//     self.iter().filter_map(|ev| {
	//         ev.and_then(|ev| ev.as_event::<Ev>().map_err(Into::into))
	//             .transpose()
	//     })
	// }

	// /// Iterate through the events using metadata to dynamically decode and skip
	// /// them, and return the first event found which decodes to the provided `Ev` type.
	// pub fn find_first<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
	//     self.find::<Ev>().next().transpose()
	// }

	// /// Iterate through the events using metadata to dynamically decode and skip
	// /// them, and return the last event found which decodes to the provided `Ev` type.
	// pub fn find_last<Ev: StaticEvent>(&self) -> Result<Option<Ev>, Error> {
	//     self.find::<Ev>().last().transpose()
	// }

	// /// Find an event that decodes to the type provided. Returns true if it was found.
	// pub fn has<Ev: StaticEvent>(&self) -> Result<bool, Error> {
	//     Ok(self.find::<Ev>().next().transpose()?.is_some())
	// }
}

/// The event details.
#[derive(Debug, Clone)]
pub struct EventDetails<Hash> {
	phase: Phase,
	/// The index of the event in the list of events in a given block.
	index: u32,
	all_bytes: Arc<[u8]>,
	// start of the bytes (phase, pallet/variant index and then fields and then topic to follow).
	start_idx: usize,
	// start of the event (ie pallet/variant index and then the fields and topic after).
	event_start_idx: usize,
	// start of the fields (ie after phase and pallet/variant index).
	event_fields_start_idx: usize,
	// end of the fields.
	event_fields_end_idx: usize,
	// end of everything (fields + topics)
	end_idx: usize,
	metadata: Metadata,
	topics: Vec<Hash>,
}

impl<Hash: Decode> EventDetails<Hash> {
	// Attempt to dynamically decode a single event from our events input.
	fn decode_from(
		metadata: Metadata,
		all_bytes: Arc<[u8]>,
		start_idx: usize,
		index: u32,
	) -> Result<EventDetails<Hash>, RpcError> {
		let input = &mut &all_bytes[start_idx..];

		let phase = Phase::decode(input).map_err(|_| RpcError::DecodeError)?;

		let event_start_idx = all_bytes.len() - input.len();

		let pallet_index = u8::decode(input).map_err(|_| RpcError::DecodeError)?;
		let variant_index = u8::decode(input).map_err(|_| RpcError::DecodeError)?;

		let event_fields_start_idx = all_bytes.len() - input.len();

		// Get metadata for the event:
		let event_pallet = metadata.pallet_by_index(pallet_index).unwrap();
		let event_variant = event_pallet
			.event_variant_by_index(variant_index)
			.ok_or(MetadataError::VariantIndexNotFound(variant_index))?;
		// Skip over the bytes belonging to this event.
		for field_metadata in &event_variant.fields {
			// Skip over the bytes for this field:
			scale_decode::visitor::decode_with_visitor(
				input,
				field_metadata.ty.id,
				metadata.types(),
				scale_decode::visitor::IgnoreVisitor,
			)
			.map_err(|_| RpcError::DecodeError)?;
		}

		// the end of the field bytes.
		let event_fields_end_idx = all_bytes.len() - input.len();

		// topics come after the event data in EventRecord.
		let topics = Vec::<Hash>::decode(input).map_err(|_| RpcError::DecodeError)?;

		// what bytes did we skip over in total, including topics.
		let end_idx = all_bytes.len() - input.len();

		Ok(EventDetails {
			phase,
			index,
			start_idx,
			event_start_idx,
			event_fields_start_idx,
			event_fields_end_idx,
			end_idx,
			all_bytes,
			metadata,
			topics,
		})
	}

	/// When was the event produced?
	pub fn phase(&self) -> Phase {
		self.phase
	}

	/// What index is this event in the stored events for this block.
	pub fn index(&self) -> u32 {
		self.index
	}

	/// The index of the pallet that the event originated from.
	pub fn pallet_index(&self) -> u8 {
		// Note: never panics; we expect these bytes to exist
		// in order that the EventDetails could be created.
		self.all_bytes[self.event_fields_start_idx - 2]
	}

	/// The index of the event variant that the event originated from.
	pub fn variant_index(&self) -> u8 {
		// Note: never panics; we expect these bytes to exist
		// in order that the EventDetails could be created.
		self.all_bytes[self.event_fields_start_idx - 1]
	}

	/// The name of the pallet from whence the Event originated.
	pub fn pallet_name(&self) -> &str {
		self.event_metadata().pallet.name()
	}

	/// The name of the event (ie the name of the variant that it corresponds to).
	pub fn variant_name(&self) -> &str {
		&self.event_metadata().variant.name
	}

	/// Fetch details from the metadata for this event.
	pub fn event_metadata(&self) -> EventMetadataDetails {
		let pallet = self
			.metadata
			.pallet_by_index(self.pallet_index())
			.expect("event pallet to be found; we did this already during decoding");
		let variant = pallet
			.event_variant_by_index(self.variant_index())
			.expect("event variant to be found; we did this already during decoding");

		EventMetadataDetails { pallet, variant }
	}

	/// Return _all_ of the bytes representing this event, which include, in order:
	/// - The phase.
	/// - Pallet and event index.
	/// - Event fields.
	/// - Event Topics.
	pub fn bytes(&self) -> &[u8] {
		&self.all_bytes[self.start_idx..self.end_idx]
	}

	/// Return the bytes representing the fields stored in this event.
	pub fn field_bytes(&self) -> &[u8] {
		&self.all_bytes[self.event_fields_start_idx..self.event_fields_end_idx]
	}

	/// Decode and provide the event fields back in the form of a [`scale_value::Composite`]
	/// type which represents the named or unnamed fields that were present in the event.
	pub fn field_values(
		&self,
	) -> Result<scale_value::Composite<scale_value::scale::TypeId>, RpcError> {
		let bytes = &mut self.field_bytes();
		let event_metadata = self.event_metadata();

		let mut fields = event_metadata
			.variant
			.fields
			.iter()
			.map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));

		use scale_decode::DecodeAsFields;
		let decoded = <scale_value::Composite<scale_value::scale::TypeId>>::decode_as_fields(
			bytes,
			&mut fields,
			self.metadata.types(),
		)
		.map_err(|_| RpcError::DecodeError)?;

		Ok(decoded)
	}

	// /// Attempt to decode these [`EventDetails`] into a type representing the event fields.
	// /// Such types are exposed in the codegen as `pallet_name::events::EventName` types.
	// pub fn as_event<E: StaticEvent>(&self) -> Result<Option<E>, RpcError> {
	// 	let ev_metadata = self.event_metadata();
	// 	if ev_metadata.pallet.name() == E::PALLET && ev_metadata.variant.name == E::EVENT {
	// 		let mut fields = ev_metadata
	// 			.variant
	// 			.fields
	// 			.iter()
	// 			.map(|f| scale_decode::Field::new(f.ty.id, f.name.as_deref()));
	// 		let decoded =
	// 			E::decode_as_fields(&mut self.field_bytes(), &mut fields, self.metadata.types())?;
	// 		Ok(Some(decoded))
	// 	} else {
	// 		Ok(None)
	// 	}
	// }

	/// Attempt to decode these [`EventDetails`] into a root event type (which includes
	/// the pallet and event enum variants as well as the event fields). A compatible
	/// type for this is exposed via static codegen as a root level `Event` type.
	// pub fn as_root_event<E: DecodeAsType>(&self) -> Result<E, RpcError> {
	// 	let bytes = &self.all_bytes[self.event_start_idx..self.event_fields_end_idx];

	// 	let decoded = E::decode_as_type(
	// 		&mut &bytes[..],
	// 		self.metadata.outer_enums().event_enum_ty(),
	// 		self.metadata.types(),
	// 	)?;

	// 	Ok(decoded)
	// }

	/// Return the topics associated with this event.
	pub fn topics(&self) -> &[Hash] {
		&self.topics
	}
}

/// Details for the given event plucked from the metadata.
pub struct EventMetadataDetails<'a> {
	pub pallet: PalletMetadata<'a>,
	pub variant: &'a scale_info::Variant<scale_info::form::PortableForm>,
}

/// A phase of a block's execution.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Decode, Encode)]
pub enum Phase {
	/// Applying an extrinsic.
	ApplyExtrinsic(u32),
	/// Finalizing the block.
	Finalization,
	/// Initializing the block.
	Initialization,
}
