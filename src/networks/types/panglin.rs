#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod bp_header_chain {
			use super::runtime_types;
			pub mod justification {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct GrandpaJustification<_0> {
					pub round: ::core::primitive::u64,
					pub commit: runtime_types::finality_grandpa::Commit<
						crate::networks::types::utils::H256,
						::core::primitive::u32,
						runtime_types::sp_consensus_grandpa::app::Signature,
						runtime_types::sp_consensus_grandpa::app::Public,
					>,
					pub votes_ancestries: ::std::vec::Vec<_0>,
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct InitializationData<_0> {
				pub header: ::std::boxed::Box<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
				pub authority_list: ::std::vec::Vec<(
					runtime_types::sp_consensus_grandpa::app::Public,
					::core::primitive::u64,
				)>,
				pub set_id: ::core::primitive::u64,
				pub operating_mode: runtime_types::bp_runtime::BasicOperatingMode,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
			}
		}
		pub mod bp_message_dispatch {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum CallOrigin<_0, _1, _2> {
				#[codec(index = 0)]
				SourceRoot,
				#[codec(index = 1)]
				TargetAccount(_0, _1, _2),
				#[codec(index = 2)]
				SourceAccount(_0),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MessagePayload<_0, _1, _2, _3> {
				pub spec_version: ::core::primitive::u32,
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
				pub origin: runtime_types::bp_message_dispatch::CallOrigin<_0, _1, _2>,
				pub dispatch_fee_payment: runtime_types::bp_runtime::messages::DispatchFeePayment,
				pub call: _3,
			}
		}
		pub mod bp_messages {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct DeliveredMessages {
				pub begin: ::core::primitive::u64,
				pub end: ::core::primitive::u64,
				pub dispatch_results: crate::networks::types::utils::bits::DecodedBits<
					::core::primitive::u8,
					crate::networks::types::utils::bits::Msb0,
				>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct InboundLaneData<_0> {
				pub relayers: ::std::vec::Vec<runtime_types::bp_messages::UnrewardedRelayer<_0>>,
				pub last_confirmed_nonce: ::core::primitive::u64,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MessageData<_0> {
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
				pub fee: _0,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MessageKey {
				pub lane_id: [::core::primitive::u8; 4usize],
				pub nonce: ::core::primitive::u64,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum MessagesOperatingMode {
				#[codec(index = 0)]
				Basic(runtime_types::bp_runtime::BasicOperatingMode),
				#[codec(index = 1)]
				RejectingOutboundMessages,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct OutboundLaneData {
				pub oldest_unpruned_nonce: ::core::primitive::u64,
				pub latest_received_nonce: ::core::primitive::u64,
				pub latest_generated_nonce: ::core::primitive::u64,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum ReceivalResult {
				#[codec(index = 0)]
				Dispatched(runtime_types::bp_runtime::messages::MessageDispatchResult),
				#[codec(index = 1)]
				InvalidNonce,
				#[codec(index = 2)]
				TooManyUnrewardedRelayers,
				#[codec(index = 3)]
				TooManyUnconfirmedMessages,
				#[codec(index = 4)]
				PreDispatchValidateFailed,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ReceivedMessages<_0> {
				pub lane: [::core::primitive::u8; 4usize],
				pub receive_results: ::std::vec::Vec<(::core::primitive::u64, _0)>,
				pub skipped_for_not_enough_weight: ::std::vec::Vec<::core::primitive::u64>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct UnrewardedRelayer<_0> {
				pub relayer: _0,
				pub messages: runtime_types::bp_messages::DeliveredMessages,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct UnrewardedRelayersState {
				pub unrewarded_relayer_entries: ::core::primitive::u64,
				pub messages_in_oldest_entry: ::core::primitive::u64,
				pub total_messages: ::core::primitive::u64,
				pub last_delivered_nonce: ::core::primitive::u64,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VerificationError {
				#[codec(index = 0)]
				MessageRejectedByOutBoundLane,
				#[codec(index = 1)]
				MessageDispatchWithBadOrigin,
				#[codec(index = 2)]
				MessageWithTooLowFee,
				#[codec(index = 3)]
				TooManyPendingMessages,
				#[codec(index = 4)]
				EmptyMessageProof,
				#[codec(index = 5)]
				InvalidMessageWeight,
				#[codec(index = 6)]
				MessagesCountMismatch,
				#[codec(index = 7)]
				MissingRequiredMessage,
				#[codec(index = 8)]
				FailedToDecodeMessage,
				#[codec(index = 9)]
				MessageTooLarge,
				#[codec(index = 10)]
				FailedToDecodeOutboundLaneData,
				#[codec(index = 11)]
				Other,
			}
		}
		pub mod bp_parachains {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct BestParaHeadHash {
				pub at_relay_block_number: ::core::primitive::u32,
				pub head_hash: crate::networks::types::utils::H256,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ParaInfo {
				pub best_head_hash: runtime_types::bp_parachains::BestParaHeadHash,
				pub next_imported_hash_position: ::core::primitive::u32,
			}
		}
		pub mod bp_polkadot_core {
			use super::runtime_types;
			pub mod parachains {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ParaHead(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ParaHeadsProof(
					pub ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ParaId(pub ::core::primitive::u32);
			}
		}
		pub mod bp_runtime {
			use super::runtime_types;
			pub mod messages {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum DispatchFeePayment {
					#[codec(index = 0)]
					AtSourceChain,
					#[codec(index = 1)]
					AtTargetChain,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct MessageDispatchResult {
					pub dispatch_result: ::core::primitive::bool,
					pub unspent_weight: runtime_types::sp_weights::weight_v2::Weight,
					pub dispatch_fee_paid_during_dispatch: ::core::primitive::bool,
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum BasicOperatingMode {
				#[codec(index = 0)]
				Normal,
				#[codec(index = 1)]
				Halted,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum OwnedBridgeModuleError {
				#[codec(index = 0)]
				Halted,
			}
		}
		pub mod bridge_runtime_common {
			use super::runtime_types;
			pub mod messages {
				use super::runtime_types;
				pub mod source {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct FromBridgedChainMessagesDeliveryProof<_0> {
						pub bridged_header_hash: _0,
						pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
						pub lane: [::core::primitive::u8; 4usize],
					}
				}
				pub mod target {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct FromBridgedChainMessagesProof<_0> {
						pub bridged_header_hash: _0,
						pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
						pub lane: [::core::primitive::u8; 4usize],
						pub nonces_start: ::core::primitive::u64,
						pub nonces_end: ::core::primitive::u64,
					}
				}
			}
		}
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					OverLimit,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 1)]
					UnsupportedVersion { message_id: [::core::primitive::u8; 32usize] },
					#[codec(index = 2)]
					ExecutedDownward {
						message_id: [::core::primitive::u8; 32usize],
						outcome: runtime_types::xcm::v3::traits::Outcome,
					},
					#[codec(index = 3)]
					WeightExhausted {
						message_id: [::core::primitive::u8; 32usize],
						remaining_weight: runtime_types::sp_weights::weight_v2::Weight,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					OverweightEnqueued {
						message_id: [::core::primitive::u8; 32usize],
						overweight_index: ::core::primitive::u64,
						required_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 5)]
					OverweightServiced {
						overweight_index: ::core::primitive::u64,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					MaxMessagesExhausted { message_id: [::core::primitive::u8; 32usize] },
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ConfigData {
				pub max_individual: runtime_types::sp_weights::weight_v2::Weight,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct PageIndexData {
				pub begin_used: ::core::primitive::u32,
				pub end_used: ::core::primitive::u32,
				pub overweight_count: ::core::primitive::u64,
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : crate :: networks :: types :: utils :: H256 , check_version : :: core :: primitive :: bool , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					OverlappingUpgrades,
					#[codec(index = 1)]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					TooBig,
					#[codec(index = 3)]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					NotScheduled,
					#[codec(index = 6)]
					NothingAuthorized,
					#[codec(index = 7)]
					Unauthorized,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ValidationFunctionStored,
					#[codec(index = 1)]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					UpgradeAuthorized { code_hash: crate::networks::types::utils::H256 },
					#[codec(index = 4)]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 5)]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: crate::networks::types::utils::H256,
					},
					#[codec(index = 6)]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct MessagingStateSnapshot { pub dmq_mqc_head : crate :: networks :: types :: utils :: H256 , pub relay_dispatch_queue_size : runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: RelayDispachQueueSize , pub ingress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , pub egress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain :: primitives :: Id , runtime_types :: polkadot_primitives :: v4 :: AbridgedHrmpChannel ,) > , }
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct RelayDispachQueueSize {
					pub remaining_count: ::core::primitive::u32,
					pub remaining_size: ::core::primitive::u32,
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct CodeUpgradeAuthorization {
				pub code_hash: crate::networks::types::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v3::traits::Outcome,
					),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Origin {
					#[codec(index = 0)]
					Relay,
					#[codec(index = 1)]
					SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					suspend_xcm_execution,
					#[codec(index = 2)]
					resume_xcm_execution,
					#[codec(index = 3)]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					update_resume_threshold { new: ::core::primitive::u32 },
					#[codec(index = 6)]
					update_threshold_weight { new: runtime_types::sp_weights::weight_v2::Weight },
					#[codec(index = 7)]
					update_weight_restrict_decay {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 8)]
					update_xcmp_max_individual_weight {
						new: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSend,
					#[codec(index = 1)]
					BadXcmOrigin,
					#[codec(index = 2)]
					BadXcm,
					#[codec(index = 3)]
					BadOverweightIndex,
					#[codec(index = 4)]
					WeightOverLimit,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Success {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 1)]
					Fail {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						error: runtime_types::xcm::v3::traits::Error,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					BadVersion {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 3)]
					BadFormat {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					XcmpMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					OverweightEnqueued {
						sender: runtime_types::polkadot_parachain::primitives::Id,
						sent_at: ::core::primitive::u32,
						index: ::core::primitive::u64,
						required: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 6)]
					OverweightServiced {
						index: ::core::primitive::u64,
						used: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct InboundChannelDetails {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
				pub message_metadata: ::std::vec::Vec<(
					::core::primitive::u32,
					runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
				)>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum InboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub weight_restrict_decay: runtime_types::sp_weights::weight_v2::Weight,
				pub xcmp_max_individual_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MessageQueueChain(pub crate::networks::types::utils::H256);
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v4::PersistedValidationData<
						crate::networks::types::utils::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
				pub horizontal_messages: crate::networks::types::utils::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
					>,
				>,
			}
		}
		pub mod darwinia_account_migration {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					migrate {
						from: crate::networks::types::utils::AccountId32,
						to: runtime_types::fp_account::AccountId20,
						signature: [::core::primitive::u8; 64usize],
					},
					#[codec(index = 1)]
					migrate_multisig {
						submitter: crate::networks::types::utils::AccountId32,
						others: ::std::vec::Vec<crate::networks::types::utils::AccountId32>,
						threshold: ::core::primitive::u16,
						to: runtime_types::fp_account::AccountId20,
						signature: [::core::primitive::u8; 64usize],
						new_multisig_params: ::core::option::Option<
							runtime_types::darwinia_account_migration::MultisigParams,
						>,
					},
					#[codec(index = 2)]
					complete_multisig_migration {
						multisig: crate::networks::types::utils::AccountId32,
						submitter: crate::networks::types::utils::AccountId32,
						signature: [::core::primitive::u8; 64usize],
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ExceedMaxVestings,
					#[codec(index = 1)]
					ExceedMaxDeposits,
					#[codec(index = 2)]
					AccountAlreadyExisted,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Migrated {
						from: crate::networks::types::utils::AccountId32,
						to: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 1)]
					NewMultisigParamsNoted {
						from: crate::networks::types::utils::AccountId32,
						to: runtime_types::darwinia_account_migration::MultisigParams,
					},
					#[codec(index = 2)]
					MultisigMigrated {
						from: crate::networks::types::utils::AccountId32,
						detail: runtime_types::darwinia_account_migration::MultisigMigrationDetail,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct AssetAccount {
				pub balance: ::core::primitive::u128,
				pub is_frozen: ::core::primitive::bool,
				pub reason: runtime_types::darwinia_account_migration::ExistenceReason,
				pub extra: (),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum ExistenceReason {
				#[codec(index = 0)]
				Consumer,
				#[codec(index = 1)]
				Sufficient,
				#[codec(index = 2)]
				DepositHeld(::core::primitive::u128),
				#[codec(index = 3)]
				DepositRefunded,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MultisigMigrationDetail {
				pub to: runtime_types::fp_account::AccountId20,
				pub members: ::std::vec::Vec<(
					crate::networks::types::utils::AccountId32,
					::core::primitive::bool,
				)>,
				pub threshold: ::core::primitive::u16,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct MultisigParams {
				pub address: runtime_types::fp_account::AccountId20,
				pub members: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
				pub threshold: ::core::primitive::u16,
			}
		}
		pub mod darwinia_asset_limit {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_foreign_asset_limit {
						asset_type:
							runtime_types::pangolin_runtime::pallets::asset_manager::AssetType,
						units_limit: ::core::primitive::u128,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AssetDoesNotExist,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					AssetLimitChanged {
						asset_type:
							runtime_types::pangolin_runtime::pallets::asset_manager::AssetType,
						units_limit: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod darwinia_common_runtime {
			use super::runtime_types;
			pub mod xcm_configs {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssetRegistrarMetadata {
					pub name: ::std::vec::Vec<::core::primitive::u8>,
					pub symbol: ::std::vec::Vec<::core::primitive::u8>,
					pub decimals: ::core::primitive::u8,
					pub is_frozen: ::core::primitive::bool,
				}
			}
		}
		pub mod darwinia_deposit {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					lock { amount: ::core::primitive::u128, months: ::core::primitive::u8 },
					#[codec(index = 1)]
					claim,
					#[codec(index = 2)]
					claim_with_penalty { id: ::core::primitive::u16 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					LockAtLeastSome,
					#[codec(index = 1)]
					LockAtLeastOneMonth,
					#[codec(index = 2)]
					LockAtMostThirtySixMonths,
					#[codec(index = 3)]
					ExceedMaxDeposits,
					#[codec(index = 4)]
					DepositNotFound,
					#[codec(index = 5)]
					DepositInUse,
					#[codec(index = 6)]
					DepositNotInUse,
					#[codec(index = 7)]
					DepositAlreadyExpired,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					DepositCreated {
						owner: runtime_types::fp_account::AccountId20,
						deposit_id: ::core::primitive::u16,
						value: ::core::primitive::u128,
						start_time: ::core::primitive::u128,
						expired_time: ::core::primitive::u128,
						kton_reward: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DepositClaimed {
						owner: runtime_types::fp_account::AccountId20,
						deposit_id: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					DepositClaimedWithPenalty {
						owner: runtime_types::fp_account::AccountId20,
						deposit_id: ::core::primitive::u16,
						kton_penalty: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Deposit {
				pub id: ::core::primitive::u16,
				pub value: ::core::primitive::u128,
				pub start_time: ::core::primitive::u128,
				pub expired_time: ::core::primitive::u128,
				pub in_use: ::core::primitive::bool,
			}
		}
		pub mod darwinia_ecdsa_authority {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_authority { new: runtime_types::fp_account::AccountId20 },
					#[codec(index = 1)]
					remove_authority { old: runtime_types::fp_account::AccountId20 },
					#[codec(index = 2)]
					swap_authority {
						old: runtime_types::fp_account::AccountId20,
						new: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 3)]
					submit_authorities_change_signature {
						signature: runtime_types::sp_core::ecdsa::Signature,
					},
					#[codec(index = 4)]
					submit_new_message_root_signature {
						signature: runtime_types::sp_core::ecdsa::Signature,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AuthorityExisted,
					#[codec(index = 1)]
					TooManyAuthorities,
					#[codec(index = 2)]
					NotAuthority,
					#[codec(index = 3)]
					AtLeastOneAuthority,
					#[codec(index = 4)]
					OnAuthoritiesChange,
					#[codec(index = 5)]
					NoAuthoritiesChange,
					#[codec(index = 6)]
					NoNewMessageRoot,
					#[codec(index = 7)]
					BadSignature,
					#[codec(index = 8)]
					AlreadySubmitted,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					CollectingAuthoritiesChangeSignatures {
						message: crate::networks::types::utils::H256,
					},
					#[codec(index = 1)]
					CollectedEnoughAuthoritiesChangeSignatures {
						operation: runtime_types::darwinia_ecdsa_authority::primitives::Operation<
							runtime_types::fp_account::AccountId20,
						>,
						threshold: ::core::option::Option<::core::primitive::u32>,
						message: crate::networks::types::utils::H256,
						signatures: ::std::vec::Vec<(
							runtime_types::fp_account::AccountId20,
							runtime_types::sp_core::ecdsa::Signature,
						)>,
					},
					#[codec(index = 2)]
					CollectingNewMessageRootSignatures {
						message: crate::networks::types::utils::H256,
					},
					#[codec(index = 3)]
					CollectedEnoughNewMessageRootSignatures {
						commitment: runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
						message: crate::networks::types::utils::H256,
						signatures: ::std::vec::Vec<(
							runtime_types::fp_account::AccountId20,
							runtime_types::sp_core::ecdsa::Signature,
						)>,
					},
				}
			}
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AuthoritiesChangeSigned {
					pub operation: runtime_types::darwinia_ecdsa_authority::primitives::Operation<
						runtime_types::fp_account::AccountId20,
					>,
					pub threshold: ::core::option::Option<::core::primitive::u32>,
					pub message: crate::networks::types::utils::H256,
					pub signatures: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::fp_account::AccountId20,
						runtime_types::sp_core::ecdsa::Signature,
					)>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Commitment {
					pub block_number: ::core::primitive::u32,
					pub message_root: crate::networks::types::utils::H256,
					pub nonce: ::core::primitive::u32,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct MessageRootSigned {
					pub commitment: runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
					pub message: crate::networks::types::utils::H256,
					pub signatures: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::fp_account::AccountId20,
						runtime_types::sp_core::ecdsa::Signature,
					)>,
					pub authorized: ::core::primitive::bool,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Operation<_0> {
					#[codec(index = 0)]
					AddMember { new: _0 },
					#[codec(index = 1)]
					RemoveMember { pre: _0, old: _0 },
					#[codec(index = 2)]
					SwapMembers { pre: _0, old: _0, new: _0 },
				}
			}
		}
		pub mod darwinia_message_gadget {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_commitment_contract {
						commitment_contract: crate::networks::types::utils::H160,
					},
				}
			}
		}
		pub mod darwinia_message_transact {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					message_transact {
						transaction:
							::std::boxed::Box<runtime_types::ethereum::transaction::TransactionV2>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					MessageTransactError(
						runtime_types::darwinia_message_transact::EvmTxErrorWrapper,
					),
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum EvmTxErrorWrapper {
				#[codec(index = 0)]
				GasLimitTooLow,
				#[codec(index = 1)]
				GasLimitTooHigh,
				#[codec(index = 2)]
				GasPriceTooLow,
				#[codec(index = 3)]
				PriorityFeeTooHigh,
				#[codec(index = 4)]
				BalanceTooLow,
				#[codec(index = 5)]
				TxNonceTooLow,
				#[codec(index = 6)]
				TxNonceTooHigh,
				#[codec(index = 7)]
				InvalidPaymentInput,
				#[codec(index = 8)]
				InvalidChainId,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum LcmpEthOrigin {
				#[codec(index = 0)]
				MessageTransact(crate::networks::types::utils::H160),
			}
		}
		pub mod darwinia_staking {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					stake {
						ring_amount: ::core::primitive::u128,
						kton_amount: ::core::primitive::u128,
						deposits: ::std::vec::Vec<::core::primitive::u16>,
					},
					#[codec(index = 1)]
					unstake {
						ring_amount: ::core::primitive::u128,
						kton_amount: ::core::primitive::u128,
						deposits: ::std::vec::Vec<::core::primitive::u16>,
					},
					#[codec(index = 2)]
					restake {
						ring_amount: ::core::primitive::u128,
						kton_amount: ::core::primitive::u128,
						deposits: ::std::vec::Vec<::core::primitive::u16>,
					},
					#[codec(index = 3)]
					claim,
					#[codec(index = 4)]
					collect { commission: runtime_types::sp_arithmetic::per_things::Perbill },
					#[codec(index = 5)]
					nominate { target: runtime_types::fp_account::AccountId20 },
					#[codec(index = 6)]
					chill,
					#[codec(index = 7)]
					set_collator_count { count: ::core::primitive::u32 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					CommissionTooHigh,
					#[codec(index = 1)]
					ExceedMaxDeposits,
					#[codec(index = 2)]
					ExceedMaxUnstakings,
					#[codec(index = 3)]
					DepositNotFound,
					#[codec(index = 4)]
					NotStaker,
					#[codec(index = 5)]
					TargetNotCollator,
					#[codec(index = 6)]
					ZeroCollatorCount,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Staked {
						staker: runtime_types::fp_account::AccountId20,
						ring_amount: ::core::primitive::u128,
						kton_amount: ::core::primitive::u128,
						deposits: ::std::vec::Vec<::core::primitive::u16>,
					},
					#[codec(index = 1)]
					Unstaked {
						staker: runtime_types::fp_account::AccountId20,
						ring_amount: ::core::primitive::u128,
						kton_amount: ::core::primitive::u128,
						deposits: ::std::vec::Vec<::core::primitive::u16>,
					},
					#[codec(index = 2)]
					CommissionUpdated {
						who: runtime_types::fp_account::AccountId20,
						commission: runtime_types::sp_arithmetic::per_things::Perbill,
					},
					#[codec(index = 3)]
					Payout {
						staker: runtime_types::fp_account::AccountId20,
						ring_amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Elected { collators: ::std::vec::Vec<runtime_types::fp_account::AccountId20> },
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Exposure<_0> {
				pub total: ::core::primitive::u32,
				pub nominators:
					::std::vec::Vec<runtime_types::darwinia_staking::IndividualExposure<_0>>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct IndividualExposure<_0> {
				pub who: _0,
				pub value: ::core::primitive::u32,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Ledger {
				pub staked_ring: ::core::primitive::u128,
				pub staked_kton: ::core::primitive::u128,
				pub staked_deposits: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u16,
				>,
				pub unstaking_ring: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
					::core::primitive::u128,
					::core::primitive::u32,
				)>,
				pub unstaking_kton: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
					::core::primitive::u128,
					::core::primitive::u32,
				)>,
				pub unstaking_deposits: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					(::core::primitive::u16, ::core::primitive::u32),
				>,
			}
		}
		pub mod ethbloom {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Bloom(pub [::core::primitive::u8; 256usize]);
		}
		pub mod ethereum {
			use super::runtime_types;
			pub mod block {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Block<_0> {
					pub header: runtime_types::ethereum::header::Header,
					pub transactions: ::std::vec::Vec<_0>,
					pub ommers: ::std::vec::Vec<runtime_types::ethereum::header::Header>,
				}
			}
			pub mod header {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Header {
					pub parent_hash: crate::networks::types::utils::H256,
					pub ommers_hash: crate::networks::types::utils::H256,
					pub beneficiary: crate::networks::types::utils::H160,
					pub state_root: crate::networks::types::utils::H256,
					pub transactions_root: crate::networks::types::utils::H256,
					pub receipts_root: crate::networks::types::utils::H256,
					pub logs_bloom: runtime_types::ethbloom::Bloom,
					pub difficulty: runtime_types::primitive_types::U256,
					pub number: runtime_types::primitive_types::U256,
					pub gas_limit: runtime_types::primitive_types::U256,
					pub gas_used: runtime_types::primitive_types::U256,
					pub timestamp: ::core::primitive::u64,
					pub extra_data: ::std::vec::Vec<::core::primitive::u8>,
					pub mix_hash: crate::networks::types::utils::H256,
					pub nonce: runtime_types::ethereum_types::hash::H64,
				}
			}
			pub mod log {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Log {
					pub address: crate::networks::types::utils::H160,
					pub topics: ::std::vec::Vec<crate::networks::types::utils::H256>,
					pub data: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod receipt {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct EIP658ReceiptData {
					pub status_code: ::core::primitive::u8,
					pub used_gas: runtime_types::primitive_types::U256,
					pub logs_bloom: runtime_types::ethbloom::Bloom,
					pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ReceiptV3 {
					#[codec(index = 0)]
					Legacy(runtime_types::ethereum::receipt::EIP658ReceiptData),
					#[codec(index = 1)]
					EIP2930(runtime_types::ethereum::receipt::EIP658ReceiptData),
					#[codec(index = 2)]
					EIP1559(runtime_types::ethereum::receipt::EIP658ReceiptData),
				}
			}
			pub mod transaction {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AccessListItem {
					pub address: crate::networks::types::utils::H160,
					pub storage_keys: ::std::vec::Vec<crate::networks::types::utils::H256>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct EIP1559Transaction {
					pub chain_id: ::core::primitive::u64,
					pub nonce: runtime_types::primitive_types::U256,
					pub max_priority_fee_per_gas: runtime_types::primitive_types::U256,
					pub max_fee_per_gas: runtime_types::primitive_types::U256,
					pub gas_limit: runtime_types::primitive_types::U256,
					pub action: runtime_types::ethereum::transaction::TransactionAction,
					pub value: runtime_types::primitive_types::U256,
					pub input: ::std::vec::Vec<::core::primitive::u8>,
					pub access_list:
						::std::vec::Vec<runtime_types::ethereum::transaction::AccessListItem>,
					pub odd_y_parity: ::core::primitive::bool,
					pub r: crate::networks::types::utils::H256,
					pub s: crate::networks::types::utils::H256,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct EIP2930Transaction {
					pub chain_id: ::core::primitive::u64,
					pub nonce: runtime_types::primitive_types::U256,
					pub gas_price: runtime_types::primitive_types::U256,
					pub gas_limit: runtime_types::primitive_types::U256,
					pub action: runtime_types::ethereum::transaction::TransactionAction,
					pub value: runtime_types::primitive_types::U256,
					pub input: ::std::vec::Vec<::core::primitive::u8>,
					pub access_list:
						::std::vec::Vec<runtime_types::ethereum::transaction::AccessListItem>,
					pub odd_y_parity: ::core::primitive::bool,
					pub r: crate::networks::types::utils::H256,
					pub s: crate::networks::types::utils::H256,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct LegacyTransaction {
					pub nonce: runtime_types::primitive_types::U256,
					pub gas_price: runtime_types::primitive_types::U256,
					pub gas_limit: runtime_types::primitive_types::U256,
					pub action: runtime_types::ethereum::transaction::TransactionAction,
					pub value: runtime_types::primitive_types::U256,
					pub input: ::std::vec::Vec<::core::primitive::u8>,
					pub signature: runtime_types::ethereum::transaction::TransactionSignature,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum TransactionAction {
					#[codec(index = 0)]
					Call(crate::networks::types::utils::H160),
					#[codec(index = 1)]
					Create,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct TransactionRecoveryId(pub ::core::primitive::u64);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct TransactionSignature {
					pub v: runtime_types::ethereum::transaction::TransactionRecoveryId,
					pub r: crate::networks::types::utils::H256,
					pub s: crate::networks::types::utils::H256,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum TransactionV2 {
					#[codec(index = 0)]
					Legacy(runtime_types::ethereum::transaction::LegacyTransaction),
					#[codec(index = 1)]
					EIP2930(runtime_types::ethereum::transaction::EIP2930Transaction),
					#[codec(index = 2)]
					EIP1559(runtime_types::ethereum::transaction::EIP1559Transaction),
				}
			}
		}
		pub mod ethereum_types {
			use super::runtime_types;
			pub mod hash {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct H64(pub [::core::primitive::u8; 8usize]);
			}
		}
		pub mod evm_core {
			use super::runtime_types;
			pub mod error {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExitError {
					#[codec(index = 0)]
					StackUnderflow,
					#[codec(index = 1)]
					StackOverflow,
					#[codec(index = 2)]
					InvalidJump,
					#[codec(index = 3)]
					InvalidRange,
					#[codec(index = 4)]
					DesignatedInvalid,
					#[codec(index = 5)]
					CallTooDeep,
					#[codec(index = 6)]
					CreateCollision,
					#[codec(index = 7)]
					CreateContractLimit,
					#[codec(index = 15)]
					InvalidCode(runtime_types::evm_core::opcode::Opcode),
					#[codec(index = 8)]
					OutOfOffset,
					#[codec(index = 9)]
					OutOfGas,
					#[codec(index = 10)]
					OutOfFund,
					#[codec(index = 11)]
					PCUnderflow,
					#[codec(index = 12)]
					CreateEmpty,
					#[codec(index = 13)]
					Other(::std::string::String),
					#[codec(index = 14)]
					MaxNonce,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExitFatal {
					#[codec(index = 0)]
					NotSupported,
					#[codec(index = 1)]
					UnhandledInterrupt,
					#[codec(index = 2)]
					CallErrorAsFatal(runtime_types::evm_core::error::ExitError),
					#[codec(index = 3)]
					Other(::std::string::String),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExitReason {
					#[codec(index = 0)]
					Succeed(runtime_types::evm_core::error::ExitSucceed),
					#[codec(index = 1)]
					Error(runtime_types::evm_core::error::ExitError),
					#[codec(index = 2)]
					Revert(runtime_types::evm_core::error::ExitRevert),
					#[codec(index = 3)]
					Fatal(runtime_types::evm_core::error::ExitFatal),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExitRevert {
					#[codec(index = 0)]
					Reverted,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExitSucceed {
					#[codec(index = 0)]
					Stopped,
					#[codec(index = 1)]
					Returned,
					#[codec(index = 2)]
					Suicided,
				}
			}
			pub mod opcode {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Opcode(pub ::core::primitive::u8);
			}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Commit<_0, _1, _2, _3> {
				pub target_hash: _0,
				pub target_number: _1,
				pub precommits: ::std::vec::Vec<
					runtime_types::finality_grandpa::SignedPrecommit<_0, _1, _2, _3>,
				>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct SignedPrecommit<_0, _1, _2, _3> {
				pub precommit: runtime_types::finality_grandpa::Precommit<_0, _1>,
				pub signature: _2,
				pub id: _3,
			}
		}
		pub mod fp_account {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct AccountId20(pub [::core::primitive::u8; 20usize]);
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct EthereumSignature(pub runtime_types::sp_core::ecdsa::Signature);
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct EthereumSigner(pub [::core::primitive::u8; 20usize]);
		}
		pub mod fp_rpc {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct TransactionStatus {
				pub transaction_hash: crate::networks::types::utils::H256,
				pub transaction_index: ::core::primitive::u32,
				pub from: crate::networks::types::utils::H160,
				pub to: ::core::option::Option<crate::networks::types::utils::H160>,
				pub contract_address: ::core::option::Option<crate::networks::types::utils::H160>,
				pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
				pub logs_bloom: runtime_types::ethbloom::Bloom,
			}
		}
		pub mod fp_self_contained {
			use super::runtime_types;
			pub mod unchecked_extrinsic {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
					pub crate::networks::types::utils::UncheckedExtrinsic<_0, _1, _2, _3>,
				);
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct DispatchInfo {
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
					pub class: runtime_types::frame_support::dispatch::DispatchClass,
					pub pays_fee: runtime_types::frame_support::dispatch::Pays,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod preimages {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Bounded<_0> {
						#[codec(index = 0)]
						Legacy {
							hash: crate::networks::types::utils::H256,
						},
						#[codec(index = 1)]
						Inline(
							runtime_types::bounded_collections::bounded_vec::BoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Lookup {
							hash: crate::networks::types::utils::H256,
							len: ::core::primitive::u32,
						},
						__Ignore(::core::marker::PhantomData<_0>),
					}
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							Debug,
							crate :: networks :: types :: ext :: codec :: Decode,
							crate :: networks :: types :: ext :: codec :: Encode,
							crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
							crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
						)]
						# [codec (crate = crate :: networks :: types :: ext :: codec)]
						#[decode_as_type(
							crate_path = "crate :: networks :: types :: ext :: scale_decode"
						)]
						#[encode_as_type(
							crate_path = "crate :: networks :: types :: ext :: scale_encode"
						)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidSpecName,
					#[codec(index = 1)]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					NonDefaultComposite,
					#[codec(index = 4)]
					NonZeroRefCount,
					#[codec(index = 5)]
					CallFiltered,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 1)]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
					},
					#[codec(index = 2)]
					CodeUpdated,
					#[codec(index = 3)]
					NewAccount { account: runtime_types::fp_account::AccountId20 },
					#[codec(index = 4)]
					KilledAccount { account: runtime_types::fp_account::AccountId20 },
					#[codec(index = 5)]
					Remarked {
						sender: runtime_types::fp_account::AccountId20,
						hash: crate::networks::types::utils::H256,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: ::core::primitive::u32,
				pub providers: ::core::primitive::u32,
				pub sufficients: ::core::primitive::u32,
				pub data: _1,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod orml_xtokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transfer {
						currency_id:
							runtime_types::pangolin_runtime::pallets::orml_xtokens::CurrencyId,
						amount: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 1)]
					transfer_multiasset {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 2)]
					transfer_with_fee {
						currency_id:
							runtime_types::pangolin_runtime::pallets::orml_xtokens::CurrencyId,
						amount: ::core::primitive::u128,
						fee: ::core::primitive::u128,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 3)]
					transfer_multiasset_with_fee {
						asset: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						fee: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAsset>,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 4)]
					transfer_multicurrencies {
						currencies: ::std::vec::Vec<(
							runtime_types::pangolin_runtime::pallets::orml_xtokens::CurrencyId,
							::core::primitive::u128,
						)>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 5)]
					transfer_multiassets {
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_item: ::core::primitive::u32,
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						dest_weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AssetHasNoReserve,
					#[codec(index = 1)]
					NotCrossChainTransfer,
					#[codec(index = 2)]
					InvalidDest,
					#[codec(index = 3)]
					NotCrossChainTransferableCurrency,
					#[codec(index = 4)]
					UnweighableMessage,
					#[codec(index = 5)]
					XcmExecutionFailed,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					InvalidAncestry,
					#[codec(index = 8)]
					InvalidAsset,
					#[codec(index = 9)]
					DestinationNotInvertible,
					#[codec(index = 10)]
					BadVersion,
					#[codec(index = 11)]
					DistinctReserveForAssetAndFee,
					#[codec(index = 12)]
					ZeroFee,
					#[codec(index = 13)]
					ZeroAmount,
					#[codec(index = 14)]
					TooManyAssetsBeingSent,
					#[codec(index = 15)]
					AssetIndexNonExistent,
					#[codec(index = 16)]
					FeeNotEnough,
					#[codec(index = 17)]
					NotSupportedMultiLocation,
					#[codec(index = 18)]
					MinXcmFeeNotDefined,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					TransferredMultiAssets {
						sender: runtime_types::fp_account::AccountId20,
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						fee: runtime_types::xcm::v3::multiasset::MultiAsset,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
				}
			}
		}
		pub mod pallet_asset_manager {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssetInfo {
					pub creator: runtime_types::fp_account::AccountId20,
					pub deposit: ::core::primitive::u128,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] register_foreign_asset { asset : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , metadata : runtime_types :: darwinia_common_runtime :: xcm_configs :: AssetRegistrarMetadata , min_amount : :: core :: primitive :: u128 , is_sufficient : :: core :: primitive :: bool , } , # [codec (index = 1)] set_asset_units_per_second { asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , units_per_second : :: core :: primitive :: u128 , num_assets_weight_hint : :: core :: primitive :: u32 , } , # [codec (index = 2)] change_existing_asset_type { asset_id : :: core :: primitive :: u64 , new_asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , num_assets_weight_hint : :: core :: primitive :: u32 , } , # [codec (index = 3)] remove_supported_asset { asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , num_assets_weight_hint : :: core :: primitive :: u32 , } , # [codec (index = 4)] remove_existing_asset_type { asset_id : :: core :: primitive :: u64 , num_assets_weight_hint : :: core :: primitive :: u32 , } , # [codec (index = 5)] register_local_asset { creator : runtime_types :: fp_account :: AccountId20 , owner : runtime_types :: fp_account :: AccountId20 , is_sufficient : :: core :: primitive :: bool , min_balance : :: core :: primitive :: u128 , } , # [codec (index = 6)] destroy_foreign_asset { asset_id : :: core :: primitive :: u64 , num_assets_weight_hint : :: core :: primitive :: u32 , } , # [codec (index = 7)] destroy_local_asset { asset_id : :: core :: primitive :: u64 , } , }
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ErrorCreatingAsset,
					#[codec(index = 1)]
					AssetAlreadyExists,
					#[codec(index = 2)]
					AssetDoesNotExist,
					#[codec(index = 3)]
					TooLowNumAssetsWeightHint,
					#[codec(index = 4)]
					LocalAssetLimitReached,
					#[codec(index = 5)]
					ErrorDestroyingAsset,
					#[codec(index = 6)]
					NotSufficientDeposit,
					#[codec(index = 7)]
					NonExistentLocalAsset,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					# [codec (index = 0)] ForeignAssetRegistered { asset_id : :: core :: primitive :: u64 , asset : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , metadata : runtime_types :: darwinia_common_runtime :: xcm_configs :: AssetRegistrarMetadata , } , # [codec (index = 1)] UnitsPerSecondChanged { asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , units_per_second : :: core :: primitive :: u128 , } , # [codec (index = 2)] ForeignAssetTypeChanged { asset_id : :: core :: primitive :: u64 , new_asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , } , # [codec (index = 3)] ForeignAssetRemoved { asset_id : :: core :: primitive :: u64 , asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , } , # [codec (index = 4)] SupportedAssetRemoved { asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , } , # [codec (index = 5)] LocalAssetRegistered { asset_id : :: core :: primitive :: u64 , creator : runtime_types :: fp_account :: AccountId20 , owner : runtime_types :: fp_account :: AccountId20 , } , # [codec (index = 6)] ForeignAssetDestroyed { asset_id : :: core :: primitive :: u64 , asset_type : runtime_types :: pangolin_runtime :: pallets :: asset_manager :: AssetType , } , # [codec (index = 7)] LocalAssetDestroyed { asset_id : :: core :: primitive :: u64 , } , }
			}
		}
		pub mod pallet_assets {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					create {
						#[codec(compact)]
						id: ::core::primitive::u64,
						admin: runtime_types::fp_account::AccountId20,
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					force_create {
						#[codec(compact)]
						id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						is_sufficient: ::core::primitive::bool,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					start_destroy {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					destroy_accounts {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					destroy_approvals {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					finish_destroy {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 6)]
					mint {
						#[codec(compact)]
						id: ::core::primitive::u64,
						beneficiary: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					burn {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					transfer {
						#[codec(compact)]
						id: ::core::primitive::u64,
						target: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					transfer_keep_alive {
						#[codec(compact)]
						id: ::core::primitive::u64,
						target: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					force_transfer {
						#[codec(compact)]
						id: ::core::primitive::u64,
						source: runtime_types::fp_account::AccountId20,
						dest: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					freeze {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 12)]
					thaw {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 13)]
					freeze_asset {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 14)]
					thaw_asset {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 15)]
					transfer_ownership {
						#[codec(compact)]
						id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 16)]
					set_team {
						#[codec(compact)]
						id: ::core::primitive::u64,
						issuer: runtime_types::fp_account::AccountId20,
						admin: runtime_types::fp_account::AccountId20,
						freezer: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 17)]
					set_metadata {
						#[codec(compact)]
						id: ::core::primitive::u64,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
					},
					#[codec(index = 18)]
					clear_metadata {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					force_set_metadata {
						#[codec(compact)]
						id: ::core::primitive::u64,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 20)]
					force_clear_metadata {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 21)]
					force_asset_status {
						#[codec(compact)]
						id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						issuer: runtime_types::fp_account::AccountId20,
						admin: runtime_types::fp_account::AccountId20,
						freezer: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						min_balance: ::core::primitive::u128,
						is_sufficient: ::core::primitive::bool,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 22)]
					approve_transfer {
						#[codec(compact)]
						id: ::core::primitive::u64,
						delegate: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 23)]
					cancel_approval {
						#[codec(compact)]
						id: ::core::primitive::u64,
						delegate: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 24)]
					force_cancel_approval {
						#[codec(compact)]
						id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						delegate: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 25)]
					transfer_approved {
						#[codec(compact)]
						id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						destination: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 26)]
					touch {
						#[codec(compact)]
						id: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					refund {
						#[codec(compact)]
						id: ::core::primitive::u64,
						allow_burn: ::core::primitive::bool,
					},
					#[codec(index = 28)]
					set_min_balance {
						#[codec(compact)]
						id: ::core::primitive::u64,
						min_balance: ::core::primitive::u128,
					},
					#[codec(index = 29)]
					touch_other {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 30)]
					refund_other {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 31)]
					block {
						#[codec(compact)]
						id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					BalanceLow,
					#[codec(index = 1)]
					NoAccount,
					#[codec(index = 2)]
					NoPermission,
					#[codec(index = 3)]
					Unknown,
					#[codec(index = 4)]
					Frozen,
					#[codec(index = 5)]
					InUse,
					#[codec(index = 6)]
					BadWitness,
					#[codec(index = 7)]
					MinBalanceZero,
					#[codec(index = 8)]
					UnavailableConsumer,
					#[codec(index = 9)]
					BadMetadata,
					#[codec(index = 10)]
					Unapproved,
					#[codec(index = 11)]
					WouldDie,
					#[codec(index = 12)]
					AlreadyExists,
					#[codec(index = 13)]
					NoDeposit,
					#[codec(index = 14)]
					WouldBurn,
					#[codec(index = 15)]
					LiveAsset,
					#[codec(index = 16)]
					AssetNotLive,
					#[codec(index = 17)]
					IncorrectStatus,
					#[codec(index = 18)]
					NotFrozen,
					#[codec(index = 19)]
					CallbackFailed,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Created {
						asset_id: ::core::primitive::u64,
						creator: runtime_types::fp_account::AccountId20,
						owner: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 1)]
					Issued {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transferred {
						asset_id: ::core::primitive::u64,
						from: runtime_types::fp_account::AccountId20,
						to: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					Burned {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					TeamChanged {
						asset_id: ::core::primitive::u64,
						issuer: runtime_types::fp_account::AccountId20,
						admin: runtime_types::fp_account::AccountId20,
						freezer: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 5)]
					OwnerChanged {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 6)]
					Frozen {
						asset_id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 7)]
					Thawed {
						asset_id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 8)]
					AssetFrozen { asset_id: ::core::primitive::u64 },
					#[codec(index = 9)]
					AssetThawed { asset_id: ::core::primitive::u64 },
					#[codec(index = 10)]
					AccountsDestroyed {
						asset_id: ::core::primitive::u64,
						accounts_destroyed: ::core::primitive::u32,
						accounts_remaining: ::core::primitive::u32,
					},
					#[codec(index = 11)]
					ApprovalsDestroyed {
						asset_id: ::core::primitive::u64,
						approvals_destroyed: ::core::primitive::u32,
						approvals_remaining: ::core::primitive::u32,
					},
					#[codec(index = 12)]
					DestructionStarted { asset_id: ::core::primitive::u64 },
					#[codec(index = 13)]
					Destroyed { asset_id: ::core::primitive::u64 },
					#[codec(index = 14)]
					ForceCreated {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 15)]
					MetadataSet {
						asset_id: ::core::primitive::u64,
						name: ::std::vec::Vec<::core::primitive::u8>,
						symbol: ::std::vec::Vec<::core::primitive::u8>,
						decimals: ::core::primitive::u8,
						is_frozen: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					MetadataCleared { asset_id: ::core::primitive::u64 },
					#[codec(index = 17)]
					ApprovedTransfer {
						asset_id: ::core::primitive::u64,
						source: runtime_types::fp_account::AccountId20,
						delegate: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					ApprovalCancelled {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						delegate: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 19)]
					TransferredApproved {
						asset_id: ::core::primitive::u64,
						owner: runtime_types::fp_account::AccountId20,
						delegate: runtime_types::fp_account::AccountId20,
						destination: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					AssetStatusChanged { asset_id: ::core::primitive::u64 },
					#[codec(index = 21)]
					AssetMinBalanceChanged {
						asset_id: ::core::primitive::u64,
						new_min_balance: ::core::primitive::u128,
					},
					#[codec(index = 22)]
					Touched {
						asset_id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
						depositor: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 23)]
					Blocked {
						asset_id: ::core::primitive::u64,
						who: runtime_types::fp_account::AccountId20,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum AccountStatus {
					#[codec(index = 0)]
					Liquid,
					#[codec(index = 1)]
					Frozen,
					#[codec(index = 2)]
					Blocked,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Approval<_0, _1> {
					pub amount: _0,
					pub deposit: _1,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssetAccount<_0, _1, _2, _3> {
					pub balance: _0,
					pub status: runtime_types::pallet_assets::types::AccountStatus,
					pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0, _3>,
					pub extra: _2,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssetDetails<_0, _1, _2> {
					pub owner: _1,
					pub issuer: _1,
					pub admin: _1,
					pub freezer: _1,
					pub supply: _0,
					pub deposit: _2,
					pub min_balance: _0,
					pub is_sufficient: ::core::primitive::bool,
					pub accounts: ::core::primitive::u32,
					pub sufficients: ::core::primitive::u32,
					pub approvals: ::core::primitive::u32,
					pub status: runtime_types::pallet_assets::types::AssetStatus,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssetMetadata<_0, _1> {
					pub deposit: _0,
					pub name: _1,
					pub symbol: _1,
					pub decimals: ::core::primitive::u8,
					pub is_frozen: ::core::primitive::bool,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum AssetStatus {
					#[codec(index = 0)]
					Live,
					#[codec(index = 1)]
					Frozen,
					#[codec(index = 2)]
					Destroying,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ExistenceReason<_0, _1> {
					#[codec(index = 0)]
					Consumer,
					#[codec(index = 1)]
					Sufficient,
					#[codec(index = 2)]
					DepositHeld(_0),
					#[codec(index = 3)]
					DepositRefunded,
					#[codec(index = 4)]
					DepositFrom(_1, _0),
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transfer_allow_death {
						dest: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					set_balance_deprecated {
						who: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						old_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					force_transfer {
						source: runtime_types::fp_account::AccountId20,
						dest: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					transfer_keep_alive {
						dest: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					transfer_all {
						dest: runtime_types::fp_account::AccountId20,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					force_unreserve {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					upgrade_accounts {
						who: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
					},
					#[codec(index = 7)]
					transfer {
						dest: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					force_set_balance {
						who: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					VestingBalance,
					#[codec(index = 1)]
					LiquidityRestrictions,
					#[codec(index = 2)]
					InsufficientBalance,
					#[codec(index = 3)]
					ExistentialDeposit,
					#[codec(index = 4)]
					Expendability,
					#[codec(index = 5)]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
					#[codec(index = 8)]
					TooManyHolds,
					#[codec(index = 9)]
					TooManyFreezes,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Endowed {
						account: runtime_types::fp_account::AccountId20,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					DustLost {
						account: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Transfer {
						from: runtime_types::fp_account::AccountId20,
						to: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					BalanceSet {
						who: runtime_types::fp_account::AccountId20,
						free: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Reserved {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					Unreserved {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					ReserveRepatriated {
						from: runtime_types::fp_account::AccountId20,
						to: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					Deposit {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					Withdraw {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					Slashed {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					Minted {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					Burned {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					Suspended {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					Restored {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 14)]
					Upgraded { who: runtime_types::fp_account::AccountId20 },
					#[codec(index = 15)]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 17)]
					Locked {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 18)]
					Unlocked {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 19)]
					Frozen {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 20)]
					Thawed {
						who: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct IdAmount<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_bridge_dispatch {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					MessageRejected(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
					),
					#[codec(index = 1)]
					MessageVersionSpecMismatch(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
						::core::primitive::u32,
						::core::primitive::u32,
					),
					#[codec(index = 2)]
					MessageWeightMismatch(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 3)]
					MessageSignatureMismatch(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
					),
					#[codec(index = 4)]
					MessageCallDecodeFailed(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
					),
					#[codec(index = 5)]
					MessageCallValidateFailed(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
						runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
					),
					#[codec(index = 6)]
					MessageDispatchPaymentFailed(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
						runtime_types::fp_account::AccountId20,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 7)]
					MessageDispatched(
						[::core::primitive::u8; 4usize],
						([::core::primitive::u8; 4usize], ::core::primitive::u64),
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
					#[codec(index = 8)]
					_Dummy,
				}
			}
		}
		pub mod pallet_bridge_grandpa {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					submit_finality_proof {
						finality_target: ::std::boxed::Box<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
						justification:
							runtime_types::bp_header_chain::justification::GrandpaJustification<
								runtime_types::sp_runtime::generic::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
							>,
					},
					#[codec(index = 1)]
					initialize {
						init_data: runtime_types::bp_header_chain::InitializationData<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
					#[codec(index = 2)]
					set_owner {
						new_owner: ::core::option::Option<runtime_types::fp_account::AccountId20>,
					},
					#[codec(index = 3)]
					set_operating_mode {
						operating_mode: runtime_types::bp_runtime::BasicOperatingMode,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidJustification,
					#[codec(index = 1)]
					InvalidAuthoritySet,
					#[codec(index = 2)]
					TooManyRequests,
					#[codec(index = 3)]
					OldHeader,
					#[codec(index = 4)]
					UnknownHeader,
					#[codec(index = 5)]
					UnsupportedScheduledChange,
					#[codec(index = 6)]
					NotInitialized,
					#[codec(index = 7)]
					AlreadyInitialized,
					#[codec(index = 8)]
					StorageRootMismatch,
					#[codec(index = 9)]
					TooManyAuthoritiesInSet,
					#[codec(index = 10)]
					TooLargeHeader,
					#[codec(index = 11)]
					BridgeModule(runtime_types::bp_runtime::OwnedBridgeModuleError),
				}
			}
			pub mod storage_types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct StoredAuthoritySet {
					pub authorities: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::sp_consensus_grandpa::app::Public,
						::core::primitive::u64,
					)>,
					pub set_id: ::core::primitive::u64,
				}
			}
		}
		pub mod pallet_bridge_messages {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					# [codec (index = 0)] set_owner { new_owner : :: core :: option :: Option < runtime_types :: fp_account :: AccountId20 > , } , # [codec (index = 1)] set_operating_mode { operating_mode : runtime_types :: bp_messages :: MessagesOperatingMode , } , # [codec (index = 2)] update_pallet_parameter { parameter : () , } , # [codec (index = 3)] send_message { lane_id : [:: core :: primitive :: u8 ; 4usize] , payload : runtime_types :: bp_message_dispatch :: MessagePayload < runtime_types :: fp_account :: AccountId20 , runtime_types :: fp_account :: EthereumSigner , runtime_types :: fp_account :: EthereumSignature , :: std :: vec :: Vec < :: core :: primitive :: u8 > > , delivery_and_dispatch_fee : :: core :: primitive :: u128 , } , # [codec (index = 5)] receive_messages_proof { relayer_id_at_bridged_chain : runtime_types :: fp_account :: AccountId20 , proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < crate :: networks :: types :: utils :: H256 > , messages_count : :: core :: primitive :: u32 , dispatch_weight : runtime_types :: sp_weights :: weight_v2 :: Weight , } , # [codec (index = 6)] receive_messages_delivery_proof { proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < crate :: networks :: types :: utils :: H256 > , relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , } , }
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotOperatingNormally,
					#[codec(index = 1)]
					MessageIsTooLarge,
					#[codec(index = 2)]
					MessageRejectedByChainVerifier(runtime_types::bp_messages::VerificationError),
					#[codec(index = 3)]
					MessageRejectedByLaneVerifier(runtime_types::bp_messages::VerificationError),
					#[codec(index = 4)]
					FailedToWithdrawMessageFee,
					#[codec(index = 5)]
					TooManyMessagesInTheProof,
					#[codec(index = 6)]
					InvalidMessagesProof,
					#[codec(index = 7)]
					InvalidMessagesDeliveryProof,
					#[codec(index = 8)]
					InvalidUnrewardedRelayers,
					#[codec(index = 9)]
					InvalidUnrewardedRelayersState,
					#[codec(index = 10)]
					TryingToConfirmMoreMessagesThanExpected,
					#[codec(index = 11)]
					BridgeModule(runtime_types::bp_runtime::OwnedBridgeModuleError),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ParameterUpdated { parameter: () },
					#[codec(index = 1)]
					MessageAccepted {
						lane_id: [::core::primitive::u8; 4usize],
						nonce: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					MessagesReceived(
						::std::vec::Vec<
							runtime_types::bp_messages::ReceivedMessages<
								runtime_types::bp_messages::ReceivalResult,
							>,
						>,
					),
					#[codec(index = 3)]
					MessagesDelivered {
						lane_id: [::core::primitive::u8; 4usize],
						messages: runtime_types::bp_messages::DeliveredMessages,
					},
				}
			}
		}
		pub mod pallet_bridge_parachains {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					submit_parachain_heads {
						at_relay_block:
							(::core::primitive::u32, crate::networks::types::utils::H256),
						parachains: ::std::vec::Vec<(
							runtime_types::bp_polkadot_core::parachains::ParaId,
							crate::networks::types::utils::H256,
						)>,
						parachain_heads_proof:
							runtime_types::bp_polkadot_core::parachains::ParaHeadsProof,
					},
					#[codec(index = 1)]
					set_owner {
						new_owner: ::core::option::Option<runtime_types::fp_account::AccountId20>,
					},
					#[codec(index = 2)]
					set_operating_mode {
						operating_mode: runtime_types::bp_runtime::BasicOperatingMode,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					UnknownRelayChainBlock,
					#[codec(index = 1)]
					InvalidRelayChainBlockNumber,
					#[codec(index = 2)]
					InvalidStorageProof,
					#[codec(index = 3)]
					UnknownParaHead,
					#[codec(index = 4)]
					StorageRootMismatch,
					#[codec(index = 5)]
					FailedToExtractStateRoot,
					#[codec(index = 6)]
					BridgeModule(runtime_types::bp_runtime::OwnedBridgeModuleError),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					UntrackedParachainRejected {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
					},
					#[codec(index = 1)]
					MissingParachainHead {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
					},
					#[codec(index = 2)]
					IncorrectParachainHeadHash {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
						parachain_head_hash: crate::networks::types::utils::H256,
						actual_parachain_head_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 3)]
					RejectedObsoleteParachainHead {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
						parachain_head_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 4)]
					RejectedLargeParachainHead {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
						parachain_head_hash: crate::networks::types::utils::H256,
						parachain_head_size: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					UpdatedParachainHead {
						parachain: runtime_types::bp_polkadot_core::parachains::ParaId,
						parachain_head_hash: crate::networks::types::utils::H256,
					},
				}
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
						prime: ::core::option::Option<runtime_types::fp_account::AccountId20>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					vote {
						proposal: crate::networks::types::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					disapprove_proposal { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 6)]
					close {
						proposal_hash: crate::networks::types::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call2 {
					#[codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
						prime: ::core::option::Option<runtime_types::fp_account::AccountId20>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					vote {
						proposal: crate::networks::types::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					disapprove_proposal { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 6)]
					close {
						proposal_hash: crate::networks::types::utils::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					NotMember,
					#[codec(index = 1)]
					DuplicateProposal,
					#[codec(index = 2)]
					ProposalMissing,
					#[codec(index = 3)]
					WrongIndex,
					#[codec(index = 4)]
					DuplicateVote,
					#[codec(index = 5)]
					AlreadyInitialized,
					#[codec(index = 6)]
					TooEarly,
					#[codec(index = 7)]
					TooManyProposals,
					#[codec(index = 8)]
					WrongProposalWeight,
					#[codec(index = 9)]
					WrongProposalLength,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error2 {
					#[codec(index = 0)]
					NotMember,
					#[codec(index = 1)]
					DuplicateProposal,
					#[codec(index = 2)]
					ProposalMissing,
					#[codec(index = 3)]
					WrongIndex,
					#[codec(index = 4)]
					DuplicateVote,
					#[codec(index = 5)]
					AlreadyInitialized,
					#[codec(index = 6)]
					TooEarly,
					#[codec(index = 7)]
					TooManyProposals,
					#[codec(index = 8)]
					WrongProposalWeight,
					#[codec(index = 9)]
					WrongProposalLength,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						account: runtime_types::fp_account::AccountId20,
						proposal_index: ::core::primitive::u32,
						proposal_hash: crate::networks::types::utils::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Voted {
						account: runtime_types::fp_account::AccountId20,
						proposal_hash: crate::networks::types::utils::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Approved { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 3)]
					Disapproved { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 4)]
					Executed {
						proposal_hash: crate::networks::types::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					MemberExecuted {
						proposal_hash: crate::networks::types::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					Closed {
						proposal_hash: crate::networks::types::utils::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event2 {
					#[codec(index = 0)]
					Proposed {
						account: runtime_types::fp_account::AccountId20,
						proposal_index: ::core::primitive::u32,
						proposal_hash: crate::networks::types::utils::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					Voted {
						account: runtime_types::fp_account::AccountId20,
						proposal_hash: crate::networks::types::utils::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Approved { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 3)]
					Disapproved { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 4)]
					Executed {
						proposal_hash: crate::networks::types::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					MemberExecuted {
						proposal_hash: crate::networks::types::utils::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					Closed {
						proposal_hash: crate::networks::types::utils::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Votes<_0, _1> {
				pub index: ::core::primitive::u32,
				pub threshold: ::core::primitive::u32,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::pangolin_runtime::RuntimeCall,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					external_propose {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::pangolin_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 5)]
					external_propose_majority {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::pangolin_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 6)]
					external_propose_default {
						proposal: runtime_types::frame_support::traits::preimages::Bounded<
							runtime_types::pangolin_runtime::RuntimeCall,
						>,
					},
					#[codec(index = 7)]
					fast_track {
						proposal_hash: crate::networks::types::utils::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					veto_external { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 9)]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					delegate {
						to: runtime_types::fp_account::AccountId20,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 11)]
					undelegate,
					#[codec(index = 12)]
					clear_public_proposals,
					#[codec(index = 13)]
					unlock { target: runtime_types::fp_account::AccountId20 },
					#[codec(index = 14)]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 15)]
					remove_other_vote {
						target: runtime_types::fp_account::AccountId20,
						index: ::core::primitive::u32,
					},
					#[codec(index = 16)]
					blacklist {
						proposal_hash: crate::networks::types::utils::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 17)]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 18)]
					set_metadata {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						maybe_hash: ::core::option::Option<crate::networks::types::utils::H256>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ValueLow,
					#[codec(index = 1)]
					ProposalMissing,
					#[codec(index = 2)]
					AlreadyCanceled,
					#[codec(index = 3)]
					DuplicateProposal,
					#[codec(index = 4)]
					ProposalBlacklisted,
					#[codec(index = 5)]
					NotSimpleMajority,
					#[codec(index = 6)]
					InvalidHash,
					#[codec(index = 7)]
					NoProposal,
					#[codec(index = 8)]
					AlreadyVetoed,
					#[codec(index = 9)]
					ReferendumInvalid,
					#[codec(index = 10)]
					NoneWaiting,
					#[codec(index = 11)]
					NotVoter,
					#[codec(index = 12)]
					NoPermission,
					#[codec(index = 13)]
					AlreadyDelegating,
					#[codec(index = 14)]
					InsufficientFunds,
					#[codec(index = 15)]
					NotDelegating,
					#[codec(index = 16)]
					VotesExist,
					#[codec(index = 17)]
					InstantNotAllowed,
					#[codec(index = 18)]
					Nonsense,
					#[codec(index = 19)]
					WrongUpperBound,
					#[codec(index = 20)]
					MaxVotesReached,
					#[codec(index = 21)]
					TooMany,
					#[codec(index = 22)]
					VotingPeriodLow,
					#[codec(index = 23)]
					PreimageNotExist,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					ExternalTabled,
					#[codec(index = 3)]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					Delegated {
						who: runtime_types::fp_account::AccountId20,
						target: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 8)]
					Undelegated { account: runtime_types::fp_account::AccountId20 },
					#[codec(index = 9)]
					Vetoed {
						who: runtime_types::fp_account::AccountId20,
						proposal_hash: crate::networks::types::utils::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					Blacklisted { proposal_hash: crate::networks::types::utils::H256 },
					#[codec(index = 11)]
					Voted {
						voter: runtime_types::fp_account::AccountId20,
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 12)]
					Seconded {
						seconder: runtime_types::fp_account::AccountId20,
						prop_index: ::core::primitive::u32,
					},
					#[codec(index = 13)]
					ProposalCanceled { prop_index: ::core::primitive::u32 },
					#[codec(index = 14)]
					MetadataSet {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 15)]
					MetadataCleared {
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 16)]
					MetadataTransferred {
						prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
						owner: runtime_types::pallet_democracy::types::MetadataOwner,
						hash: crate::networks::types::utils::H256,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum MetadataOwner {
					#[codec(index = 0)]
					External,
					#[codec(index = 1)]
					Proposal(::core::primitive::u32),
					#[codec(index = 2)]
					Referendum(::core::primitive::u32),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Voting<_0, _1, _2> {
					#[codec(index = 0)]
					Direct {
						votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec(index = 1)]
					Delegating {
						balance: _0,
						target: _1,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
				}
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
		}
		pub mod pallet_elections_phragmen {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					vote {
						votes: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					remove_voter,
					#[codec(index = 2)]
					submit_candidacy {
						#[codec(compact)]
						candidate_count: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					renounce_candidacy {
						renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
					},
					#[codec(index = 4)]
					remove_member {
						who: runtime_types::fp_account::AccountId20,
						slash_bond: ::core::primitive::bool,
						rerun_election: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					clean_defunct_voters {
						num_voters: ::core::primitive::u32,
						num_defunct: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					UnableToVote,
					#[codec(index = 1)]
					NoVotes,
					#[codec(index = 2)]
					TooManyVotes,
					#[codec(index = 3)]
					MaximumVotesExceeded,
					#[codec(index = 4)]
					LowBalance,
					#[codec(index = 5)]
					UnableToPayBond,
					#[codec(index = 6)]
					MustBeVoter,
					#[codec(index = 7)]
					DuplicatedCandidate,
					#[codec(index = 8)]
					TooManyCandidates,
					#[codec(index = 9)]
					MemberSubmit,
					#[codec(index = 10)]
					RunnerUpSubmit,
					#[codec(index = 11)]
					InsufficientCandidateFunds,
					#[codec(index = 12)]
					NotMember,
					#[codec(index = 13)]
					InvalidWitnessData,
					#[codec(index = 14)]
					InvalidVoteCount,
					#[codec(index = 15)]
					InvalidRenouncing,
					#[codec(index = 16)]
					InvalidReplacement,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewTerm {
						new_members: ::std::vec::Vec<(
							runtime_types::fp_account::AccountId20,
							::core::primitive::u128,
						)>,
					},
					#[codec(index = 1)]
					EmptyTerm,
					#[codec(index = 2)]
					ElectionError,
					#[codec(index = 3)]
					MemberKicked { member: runtime_types::fp_account::AccountId20 },
					#[codec(index = 4)]
					Renounced { candidate: runtime_types::fp_account::AccountId20 },
					#[codec(index = 5)]
					CandidateSlashed {
						candidate: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					SeatHolderSlashed {
						seat_holder: runtime_types::fp_account::AccountId20,
						amount: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum Renouncing {
				#[codec(index = 0)]
				Member,
				#[codec(index = 1)]
				RunnerUp,
				#[codec(index = 2)]
				Candidate(#[codec(compact)] ::core::primitive::u32),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct SeatHolder<_0, _1> {
				pub who: _0,
				pub stake: _1,
				pub deposit: _1,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Voter<_0, _1> {
				pub votes: ::std::vec::Vec<_0>,
				pub stake: _1,
				pub deposit: _1,
			}
		}
		pub mod pallet_ethereum {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transact { transaction: runtime_types::ethereum::transaction::TransactionV2 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidSignature,
					#[codec(index = 1)]
					PreLogExists,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Executed {
						from: crate::networks::types::utils::H160,
						to: crate::networks::types::utils::H160,
						transaction_hash: crate::networks::types::utils::H256,
						exit_reason: runtime_types::evm_core::error::ExitReason,
						extra_data: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RawOrigin {
				#[codec(index = 0)]
				EthereumTransaction(crate::networks::types::utils::H160),
			}
		}
		pub mod pallet_ethereum_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					transact {
						xcm_transaction:
							runtime_types::xcm_primitives::ethereum_xcm::EthereumXcmTransaction,
					},
					#[codec(index = 1)]
					transact_through_proxy {
						transact_as: crate::networks::types::utils::H160,
						xcm_transaction:
							runtime_types::xcm_primitives::ethereum_xcm::EthereumXcmTransaction,
					},
					#[codec(index = 2)]
					suspend_ethereum_xcm_execution,
					#[codec(index = 3)]
					resume_ethereum_xcm_execution,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					EthereumXcmExecutionSuspended,
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RawOrigin {
				#[codec(index = 0)]
				XcmEthereumTransaction(crate::networks::types::utils::H160),
			}
		}
		pub mod pallet_evm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					withdraw {
						address: crate::networks::types::utils::H160,
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					call {
						source: crate::networks::types::utils::H160,
						target: crate::networks::types::utils::H160,
						input: ::std::vec::Vec<::core::primitive::u8>,
						value: runtime_types::primitive_types::U256,
						gas_limit: ::core::primitive::u64,
						max_fee_per_gas: runtime_types::primitive_types::U256,
						max_priority_fee_per_gas:
							::core::option::Option<runtime_types::primitive_types::U256>,
						nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
						access_list: ::std::vec::Vec<(
							crate::networks::types::utils::H160,
							::std::vec::Vec<crate::networks::types::utils::H256>,
						)>,
					},
					#[codec(index = 2)]
					create {
						source: crate::networks::types::utils::H160,
						init: ::std::vec::Vec<::core::primitive::u8>,
						value: runtime_types::primitive_types::U256,
						gas_limit: ::core::primitive::u64,
						max_fee_per_gas: runtime_types::primitive_types::U256,
						max_priority_fee_per_gas:
							::core::option::Option<runtime_types::primitive_types::U256>,
						nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
						access_list: ::std::vec::Vec<(
							crate::networks::types::utils::H160,
							::std::vec::Vec<crate::networks::types::utils::H256>,
						)>,
					},
					#[codec(index = 3)]
					create2 {
						source: crate::networks::types::utils::H160,
						init: ::std::vec::Vec<::core::primitive::u8>,
						salt: crate::networks::types::utils::H256,
						value: runtime_types::primitive_types::U256,
						gas_limit: ::core::primitive::u64,
						max_fee_per_gas: runtime_types::primitive_types::U256,
						max_priority_fee_per_gas:
							::core::option::Option<runtime_types::primitive_types::U256>,
						nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
						access_list: ::std::vec::Vec<(
							crate::networks::types::utils::H160,
							::std::vec::Vec<crate::networks::types::utils::H256>,
						)>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					BalanceLow,
					#[codec(index = 1)]
					FeeOverflow,
					#[codec(index = 2)]
					PaymentOverflow,
					#[codec(index = 3)]
					WithdrawFailed,
					#[codec(index = 4)]
					GasPriceTooLow,
					#[codec(index = 5)]
					InvalidNonce,
					#[codec(index = 6)]
					GasLimitTooLow,
					#[codec(index = 7)]
					GasLimitTooHigh,
					#[codec(index = 8)]
					Undefined,
					#[codec(index = 9)]
					Reentrancy,
					#[codec(index = 10)]
					TransactionMustComeFromEOA,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Log { log: runtime_types::ethereum::log::Log },
					#[codec(index = 1)]
					Created { address: crate::networks::types::utils::H160 },
					#[codec(index = 2)]
					CreatedFailed { address: crate::networks::types::utils::H160 },
					#[codec(index = 3)]
					Executed { address: crate::networks::types::utils::H160 },
					#[codec(index = 4)]
					ExecutedFailed { address: crate::networks::types::utils::H160 },
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct CodeMetadata {
				pub size: ::core::primitive::u64,
				pub hash: crate::networks::types::utils::H256,
			}
		}
		pub mod pallet_fee_market {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					enroll_and_lock_collateral {
						lock_collateral: ::core::primitive::u128,
						relay_fee: ::core::option::Option<::core::primitive::u128>,
					},
					#[codec(index = 1)]
					increase_locked_collateral { new_collateral: ::core::primitive::u128 },
					#[codec(index = 2)]
					decrease_locked_collateral { new_collateral: ::core::primitive::u128 },
					#[codec(index = 3)]
					update_relay_fee { new_fee: ::core::primitive::u128 },
					#[codec(index = 4)]
					cancel_enrollment,
					#[codec(index = 5)]
					set_slash_protect { slash_protect: ::core::primitive::u128 },
					#[codec(index = 6)]
					set_assigned_relayers_number { number: ::core::primitive::u32 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InsufficientBalance,
					#[codec(index = 1)]
					AlreadyEnrolled,
					#[codec(index = 2)]
					NotEnrolled,
					#[codec(index = 3)]
					CollateralTooLow,
					#[codec(index = 4)]
					NewCollateralShouldLargerThanBefore,
					#[codec(index = 5)]
					NewCollateralShouldLessThanBefore,
					#[codec(index = 6)]
					StillHasOrdersNotConfirmed,
					#[codec(index = 7)]
					RelayFeeTooLow,
					#[codec(index = 8)]
					OccupiedRelayer,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Enroll(
						runtime_types::fp_account::AccountId20,
						::core::primitive::u128,
						::core::primitive::u128,
					),
					#[codec(index = 1)]
					UpdateLockedCollateral(
						runtime_types::fp_account::AccountId20,
						::core::primitive::u128,
					),
					#[codec(index = 2)]
					UpdateRelayFee(runtime_types::fp_account::AccountId20, ::core::primitive::u128),
					#[codec(index = 3)]
					CancelEnrollment(runtime_types::fp_account::AccountId20),
					#[codec(index = 4)]
					UpdateCollateralSlashProtect(::core::primitive::u128),
					#[codec(index = 5)]
					UpdateAssignedRelayersNumber(::core::primitive::u32),
					#[codec(index = 6)]
					FeeMarketSlash(
						runtime_types::pallet_fee_market::types::SlashReport<
							runtime_types::fp_account::AccountId20,
							::core::primitive::u32,
							::core::primitive::u128,
						>,
					),
					#[codec(index = 7)]
					OrderCreated(
						[::core::primitive::u8; 4usize],
						::core::primitive::u64,
						::core::primitive::u128,
						::std::vec::Vec<runtime_types::fp_account::AccountId20>,
						::core::option::Option<::core::primitive::u32>,
					),
					#[codec(index = 8)]
					OrderReward(
						[::core::primitive::u8; 4usize],
						::core::primitive::u64,
						runtime_types::pallet_fee_market::s2s::payment::RewardItem<
							runtime_types::fp_account::AccountId20,
							::core::primitive::u128,
						>,
					),
				}
			}
			pub mod s2s {
				use super::runtime_types;
				pub mod payment {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct RewardItem<_0, _1> {
						pub to_assigned_relayers: crate::networks::types::utils::KeyedVec<_0, _1>,
						pub to_treasury: ::core::option::Option<_1>,
						pub to_message_relayer: ::core::option::Option<(_0, _1)>,
						pub to_confirm_relayer: ::core::option::Option<(_0, _1)>,
					}
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AssignedRelayer<_0, _1, _2> {
					pub id: _0,
					pub fee: _2,
					pub valid_range: ::core::ops::Range<_1>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Order<_0, _1, _2> {
					pub lane: [::core::primitive::u8; 4usize],
					pub message: ::core::primitive::u64,
					pub sent_time: _1,
					pub confirm_time: ::core::option::Option<_1>,
					pub collateral_per_assigned_relayer: _2,
					pub assigned_relayers: ::std::vec::Vec<
						runtime_types::pallet_fee_market::types::AssignedRelayer<_0, _1, _2>,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Relayer<_0, _1> {
					pub id: _0,
					pub collateral: _1,
					pub fee: _1,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct SlashReport<_0, _1, _2> {
					pub lane: [::core::primitive::u8; 4usize],
					pub message: ::core::primitive::u64,
					pub sent_time: _1,
					pub confirm_time: ::core::option::Option<_1>,
					pub delay_time: ::core::option::Option<_1>,
					pub account_id: _0,
					pub amount: _2,
				}
			}
		}
		pub mod pallet_identity {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_registrar { account: runtime_types::fp_account::AccountId20 },
					#[codec(index = 1)]
					set_identity {
						info:
							::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
					},
					#[codec(index = 2)]
					set_subs {
						subs: ::std::vec::Vec<(
							runtime_types::fp_account::AccountId20,
							runtime_types::pallet_identity::types::Data,
						)>,
					},
					#[codec(index = 3)]
					clear_identity,
					#[codec(index = 4)]
					request_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						#[codec(compact)]
						max_fee: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					cancel_request { reg_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					set_fee {
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						fee: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					set_account_id {
						#[codec(compact)]
						index: ::core::primitive::u32,
						new: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 8)]
					set_fields {
						#[codec(compact)]
						index: ::core::primitive::u32,
						fields: runtime_types::pallet_identity::types::BitFlags<
							runtime_types::pallet_identity::types::IdentityField,
						>,
					},
					#[codec(index = 9)]
					provide_judgement {
						#[codec(compact)]
						reg_index: ::core::primitive::u32,
						target: runtime_types::fp_account::AccountId20,
						judgement: runtime_types::pallet_identity::types::Judgement<
							::core::primitive::u128,
						>,
						identity: crate::networks::types::utils::H256,
					},
					#[codec(index = 10)]
					kill_identity { target: runtime_types::fp_account::AccountId20 },
					#[codec(index = 11)]
					add_sub {
						sub: runtime_types::fp_account::AccountId20,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 12)]
					rename_sub {
						sub: runtime_types::fp_account::AccountId20,
						data: runtime_types::pallet_identity::types::Data,
					},
					#[codec(index = 13)]
					remove_sub { sub: runtime_types::fp_account::AccountId20 },
					#[codec(index = 14)]
					quit_sub,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooManySubAccounts,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					NotNamed,
					#[codec(index = 3)]
					EmptyIndex,
					#[codec(index = 4)]
					FeeChanged,
					#[codec(index = 5)]
					NoIdentity,
					#[codec(index = 6)]
					StickyJudgement,
					#[codec(index = 7)]
					JudgementGiven,
					#[codec(index = 8)]
					InvalidJudgement,
					#[codec(index = 9)]
					InvalidIndex,
					#[codec(index = 10)]
					InvalidTarget,
					#[codec(index = 11)]
					TooManyFields,
					#[codec(index = 12)]
					TooManyRegistrars,
					#[codec(index = 13)]
					AlreadyClaimed,
					#[codec(index = 14)]
					NotSub,
					#[codec(index = 15)]
					NotOwned,
					#[codec(index = 16)]
					JudgementForDifferentIdentity,
					#[codec(index = 17)]
					JudgementPaymentFailed,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					IdentitySet { who: runtime_types::fp_account::AccountId20 },
					#[codec(index = 1)]
					IdentityCleared {
						who: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					IdentityKilled {
						who: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					JudgementRequested {
						who: runtime_types::fp_account::AccountId20,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					JudgementUnrequested {
						who: runtime_types::fp_account::AccountId20,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					JudgementGiven {
						target: runtime_types::fp_account::AccountId20,
						registrar_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					RegistrarAdded { registrar_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					SubIdentityAdded {
						sub: runtime_types::fp_account::AccountId20,
						main: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					SubIdentityRemoved {
						sub: runtime_types::fp_account::AccountId20,
						main: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					SubIdentityRevoked {
						sub: runtime_types::fp_account::AccountId20,
						main: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BitFlags<_0>(
					pub ::core::primitive::u64,
					#[codec(skip)] pub ::core::marker::PhantomData<_0>,
				);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Data {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Raw0([::core::primitive::u8; 0usize]),
					#[codec(index = 2)]
					Raw1([::core::primitive::u8; 1usize]),
					#[codec(index = 3)]
					Raw2([::core::primitive::u8; 2usize]),
					#[codec(index = 4)]
					Raw3([::core::primitive::u8; 3usize]),
					#[codec(index = 5)]
					Raw4([::core::primitive::u8; 4usize]),
					#[codec(index = 6)]
					Raw5([::core::primitive::u8; 5usize]),
					#[codec(index = 7)]
					Raw6([::core::primitive::u8; 6usize]),
					#[codec(index = 8)]
					Raw7([::core::primitive::u8; 7usize]),
					#[codec(index = 9)]
					Raw8([::core::primitive::u8; 8usize]),
					#[codec(index = 10)]
					Raw9([::core::primitive::u8; 9usize]),
					#[codec(index = 11)]
					Raw10([::core::primitive::u8; 10usize]),
					#[codec(index = 12)]
					Raw11([::core::primitive::u8; 11usize]),
					#[codec(index = 13)]
					Raw12([::core::primitive::u8; 12usize]),
					#[codec(index = 14)]
					Raw13([::core::primitive::u8; 13usize]),
					#[codec(index = 15)]
					Raw14([::core::primitive::u8; 14usize]),
					#[codec(index = 16)]
					Raw15([::core::primitive::u8; 15usize]),
					#[codec(index = 17)]
					Raw16([::core::primitive::u8; 16usize]),
					#[codec(index = 18)]
					Raw17([::core::primitive::u8; 17usize]),
					#[codec(index = 19)]
					Raw18([::core::primitive::u8; 18usize]),
					#[codec(index = 20)]
					Raw19([::core::primitive::u8; 19usize]),
					#[codec(index = 21)]
					Raw20([::core::primitive::u8; 20usize]),
					#[codec(index = 22)]
					Raw21([::core::primitive::u8; 21usize]),
					#[codec(index = 23)]
					Raw22([::core::primitive::u8; 22usize]),
					#[codec(index = 24)]
					Raw23([::core::primitive::u8; 23usize]),
					#[codec(index = 25)]
					Raw24([::core::primitive::u8; 24usize]),
					#[codec(index = 26)]
					Raw25([::core::primitive::u8; 25usize]),
					#[codec(index = 27)]
					Raw26([::core::primitive::u8; 26usize]),
					#[codec(index = 28)]
					Raw27([::core::primitive::u8; 27usize]),
					#[codec(index = 29)]
					Raw28([::core::primitive::u8; 28usize]),
					#[codec(index = 30)]
					Raw29([::core::primitive::u8; 29usize]),
					#[codec(index = 31)]
					Raw30([::core::primitive::u8; 30usize]),
					#[codec(index = 32)]
					Raw31([::core::primitive::u8; 31usize]),
					#[codec(index = 33)]
					Raw32([::core::primitive::u8; 32usize]),
					#[codec(index = 34)]
					BlakeTwo256([::core::primitive::u8; 32usize]),
					#[codec(index = 35)]
					Sha256([::core::primitive::u8; 32usize]),
					#[codec(index = 36)]
					Keccak256([::core::primitive::u8; 32usize]),
					#[codec(index = 37)]
					ShaThree256([::core::primitive::u8; 32usize]),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum IdentityField {
					#[codec(index = 1)]
					Display,
					#[codec(index = 2)]
					Legal,
					#[codec(index = 4)]
					Web,
					#[codec(index = 8)]
					Riot,
					#[codec(index = 16)]
					Email,
					#[codec(index = 32)]
					PgpFingerprint,
					#[codec(index = 64)]
					Image,
					#[codec(index = 128)]
					Twitter,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct IdentityInfo {
					pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::pallet_identity::types::Data,
						runtime_types::pallet_identity::types::Data,
					)>,
					pub display: runtime_types::pallet_identity::types::Data,
					pub legal: runtime_types::pallet_identity::types::Data,
					pub web: runtime_types::pallet_identity::types::Data,
					pub riot: runtime_types::pallet_identity::types::Data,
					pub email: runtime_types::pallet_identity::types::Data,
					pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
					pub image: runtime_types::pallet_identity::types::Data,
					pub twitter: runtime_types::pallet_identity::types::Data,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Judgement<_0> {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					FeePaid(_0),
					#[codec(index = 2)]
					Reasonable,
					#[codec(index = 3)]
					KnownGood,
					#[codec(index = 4)]
					OutOfDate,
					#[codec(index = 5)]
					LowQuality,
					#[codec(index = 6)]
					Erroneous,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct RegistrarInfo<_0, _1> {
					pub account: _1,
					pub fee: _0,
					pub fields: runtime_types::pallet_identity::types::BitFlags<
						runtime_types::pallet_identity::types::IdentityField,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Registration<_0> {
					pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u32,
						runtime_types::pallet_identity::types::Judgement<_0>,
					)>,
					pub deposit: _0,
					pub info: runtime_types::pallet_identity::types::IdentityInfo,
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					add_member { who: runtime_types::fp_account::AccountId20 },
					#[codec(index = 1)]
					remove_member { who: runtime_types::fp_account::AccountId20 },
					#[codec(index = 2)]
					swap_member {
						remove: runtime_types::fp_account::AccountId20,
						add: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 3)]
					reset_members {
						members: ::std::vec::Vec<runtime_types::fp_account::AccountId20>,
					},
					#[codec(index = 4)]
					change_key { new: runtime_types::fp_account::AccountId20 },
					#[codec(index = 5)]
					set_prime { who: runtime_types::fp_account::AccountId20 },
					#[codec(index = 6)]
					clear_prime,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					AlreadyMember,
					#[codec(index = 1)]
					NotMember,
					#[codec(index = 2)]
					TooManyMembers,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					MemberAdded,
					#[codec(index = 1)]
					MemberRemoved,
					#[codec(index = 2)]
					MembersSwapped,
					#[codec(index = 3)]
					MembersReset,
					#[codec(index = 4)]
					KeyChanged,
					#[codec(index = 5)]
					Dummy,
				}
			}
		}
		pub mod pallet_preimage {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					note_preimage { bytes: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					unnote_preimage { hash: crate::networks::types::utils::H256 },
					#[codec(index = 2)]
					request_preimage { hash: crate::networks::types::utils::H256 },
					#[codec(index = 3)]
					unrequest_preimage { hash: crate::networks::types::utils::H256 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooBig,
					#[codec(index = 1)]
					AlreadyNoted,
					#[codec(index = 2)]
					NotAuthorized,
					#[codec(index = 3)]
					NotNoted,
					#[codec(index = 4)]
					Requested,
					#[codec(index = 5)]
					NotRequested,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Noted { hash: crate::networks::types::utils::H256 },
					#[codec(index = 1)]
					Requested { hash: crate::networks::types::utils::H256 },
					#[codec(index = 2)]
					Cleared { hash: crate::networks::types::utils::H256 },
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RequestStatus<_0, _1> {
				#[codec(index = 0)]
				Unrequested { deposit: (_0, _1), len: ::core::primitive::u32 },
				#[codec(index = 1)]
				Requested {
					deposit: ::core::option::Option<(_0, _1)>,
					count: ::core::primitive::u32,
					len: ::core::option::Option<::core::primitive::u32>,
				},
			}
		}
		pub mod pallet_proxy {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					proxy {
						real: runtime_types::fp_account::AccountId20,
						force_proxy_type: ::core::option::Option<
							runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					add_proxy {
						delegate: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					remove_proxy {
						delegate: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					remove_proxies,
					#[codec(index = 4)]
					create_pure {
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec(index = 5)]
					kill_pure {
						spawner: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						index: ::core::primitive::u16,
						#[codec(compact)]
						height: ::core::primitive::u32,
						#[codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					announce {
						real: runtime_types::fp_account::AccountId20,
						call_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 7)]
					remove_announcement {
						real: runtime_types::fp_account::AccountId20,
						call_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 8)]
					reject_announcement {
						delegate: runtime_types::fp_account::AccountId20,
						call_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 9)]
					proxy_announced {
						delegate: runtime_types::fp_account::AccountId20,
						real: runtime_types::fp_account::AccountId20,
						force_proxy_type: ::core::option::Option<
							runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						>,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooMany,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					NotProxy,
					#[codec(index = 3)]
					Unproxyable,
					#[codec(index = 4)]
					Duplicate,
					#[codec(index = 5)]
					NoPermission,
					#[codec(index = 6)]
					Unannounced,
					#[codec(index = 7)]
					NoSelfProxy,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					PureCreated {
						pure: runtime_types::fp_account::AccountId20,
						who: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					Announced {
						real: runtime_types::fp_account::AccountId20,
						proxy: runtime_types::fp_account::AccountId20,
						call_hash: crate::networks::types::utils::H256,
					},
					#[codec(index = 3)]
					ProxyAdded {
						delegator: runtime_types::fp_account::AccountId20,
						delegatee: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					ProxyRemoved {
						delegator: runtime_types::fp_account::AccountId20,
						delegatee: runtime_types::fp_account::AccountId20,
						proxy_type: runtime_types::pangolin_runtime::pallets::proxy::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Announcement<_0, _1, _2> {
				pub real: _0,
				pub call_hash: _1,
				pub height: _2,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ProxyDefinition<_0, _1, _2> {
				pub delegate: _0,
				pub proxy_type: _1,
				pub delay: _2,
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 1)]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					schedule_named {
						id: [::core::primitive::u8; 32usize],
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					cancel_named { id: [::core::primitive::u8; 32usize] },
					#[codec(index = 4)]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					schedule_named_after {
						id: [::core::primitive::u8; 32usize],
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSchedule,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					RescheduleNoChange,
					#[codec(index = 4)]
					Named,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Scheduled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					Dispatched {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					CallUnavailable {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 4)]
					PeriodicFailed {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
					#[codec(index = 5)]
					PermanentlyOverweight {
						task: (::core::primitive::u32, ::core::primitive::u32),
						id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Scheduled<_0, _1, _2, _3, _4> {
				pub maybe_id: ::core::option::Option<_0>,
				pub priority: ::core::primitive::u8,
				pub call: _1,
				pub maybe_periodic: ::core::option::Option<(_2, _2)>,
				pub origin: _3,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set_keys {
						keys: runtime_types::pangolin_runtime::pallets::session::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					purge_keys,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InvalidProof,
					#[codec(index = 1)]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					DuplicatedKey,
					#[codec(index = 3)]
					NoKeys,
					#[codec(index = 4)]
					NoAccount,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					sudo { call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall> },
					#[codec(index = 1)]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					set_key { new: runtime_types::fp_account::AccountId20 },
					#[codec(index = 3)]
					sudo_as {
						who: runtime_types::fp_account::AccountId20,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					RequireSudo,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					KeyChanged {
						old_sudoer: ::core::option::Option<runtime_types::fp_account::AccountId20>,
					},
					#[codec(index = 2)]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_tips {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					report_awesome {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 1)]
					retract_tip { hash: crate::networks::types::utils::H256 },
					#[codec(index = 2)]
					tip_new {
						reason: ::std::vec::Vec<::core::primitive::u8>,
						who: runtime_types::fp_account::AccountId20,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					tip {
						hash: crate::networks::types::utils::H256,
						#[codec(compact)]
						tip_value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					close_tip { hash: crate::networks::types::utils::H256 },
					#[codec(index = 5)]
					slash_tip { hash: crate::networks::types::utils::H256 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					ReasonTooBig,
					#[codec(index = 1)]
					AlreadyKnown,
					#[codec(index = 2)]
					UnknownTip,
					#[codec(index = 3)]
					NotFinder,
					#[codec(index = 4)]
					StillOpen,
					#[codec(index = 5)]
					Premature,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					NewTip { tip_hash: crate::networks::types::utils::H256 },
					#[codec(index = 1)]
					TipClosing { tip_hash: crate::networks::types::utils::H256 },
					#[codec(index = 2)]
					TipClosed {
						tip_hash: crate::networks::types::utils::H256,
						who: runtime_types::fp_account::AccountId20,
						payout: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					TipRetracted { tip_hash: crate::networks::types::utils::H256 },
					#[codec(index = 4)]
					TipSlashed {
						tip_hash: crate::networks::types::utils::H256,
						finder: runtime_types::fp_account::AccountId20,
						deposit: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct OpenTip<_0, _1, _2, _3> {
				pub reason: _3,
				pub who: _0,
				pub finder: _0,
				pub deposit: _1,
				pub closes: ::core::option::Option<_2>,
				pub tips: ::std::vec::Vec<(_0, _1)>,
				pub finders_fee: ::core::primitive::bool,
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					TransactionFeePaid {
						who: runtime_types::fp_account::AccountId20,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 1)]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					spend {
						#[codec(compact)]
						amount: ::core::primitive::u128,
						beneficiary: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 4)]
					remove_approval {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					InvalidIndex,
					#[codec(index = 2)]
					TooManyApprovals,
					#[codec(index = 3)]
					InsufficientPermission,
					#[codec(index = 4)]
					ProposalNotApproved,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Proposed { proposal_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					Spending { budget_remaining: ::core::primitive::u128 },
					#[codec(index = 2)]
					Awarded {
						proposal_index: ::core::primitive::u32,
						award: ::core::primitive::u128,
						account: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 3)]
					Rejected {
						proposal_index: ::core::primitive::u32,
						slashed: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					Burnt { burnt_funds: ::core::primitive::u128 },
					#[codec(index = 5)]
					Rollover { rollover_balance: ::core::primitive::u128 },
					#[codec(index = 6)]
					Deposit { value: ::core::primitive::u128 },
					#[codec(index = 7)]
					SpendApproved {
						proposal_index: ::core::primitive::u32,
						amount: ::core::primitive::u128,
						beneficiary: runtime_types::fp_account::AccountId20,
					},
					#[codec(index = 8)]
					UpdatedInactive {
						reactivated: ::core::primitive::u128,
						deactivated: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					batch { calls: ::std::vec::Vec<runtime_types::pangolin_runtime::RuntimeCall> },
					#[codec(index = 1)]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 2)]
					batch_all {
						calls: ::std::vec::Vec<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 3)]
					dispatch_as {
						as_origin: ::std::boxed::Box<runtime_types::pangolin_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					force_batch {
						calls: ::std::vec::Vec<runtime_types::pangolin_runtime::RuntimeCall>,
					},
					#[codec(index = 5)]
					with_weight {
						call: ::std::boxed::Box<runtime_types::pangolin_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					TooManyCalls,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					BatchCompleted,
					#[codec(index = 2)]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					ItemCompleted,
					#[codec(index = 4)]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec(index = 5)]
					DispatchedAs {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 4)]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 9)]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 10)]
					force_suspension { suspended: ::core::primitive::bool },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Error {
					#[codec(index = 0)]
					Unreachable,
					#[codec(index = 1)]
					SendFailure,
					#[codec(index = 2)]
					Filtered,
					#[codec(index = 3)]
					UnweighableMessage,
					#[codec(index = 4)]
					DestinationNotInvertible,
					#[codec(index = 5)]
					Empty,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					TooManyAssets,
					#[codec(index = 8)]
					InvalidOrigin,
					#[codec(index = 9)]
					BadVersion,
					#[codec(index = 10)]
					BadLocation,
					#[codec(index = 11)]
					NoSubscription,
					#[codec(index = 12)]
					AlreadySubscribed,
					#[codec(index = 13)]
					InvalidAsset,
					#[codec(index = 14)]
					LowBalance,
					#[codec(index = 15)]
					TooManyLocks,
					#[codec(index = 16)]
					AccountNotSovereign,
					#[codec(index = 17)]
					FeesNotMet,
					#[codec(index = 18)]
					LockNotFound,
					#[codec(index = 19)]
					InUse,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Event {
					#[codec(index = 0)]
					Attempted(runtime_types::xcm::v3::traits::Outcome),
					#[codec(index = 1)]
					Sent(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::Xcm,
					),
					#[codec(index = 2)]
					UnexpectedResponse(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v3::Response),
					#[codec(index = 4)]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						runtime_types::sp_weights::weight_v2::Weight,
						runtime_types::sp_weights::weight_v2::Weight,
					),
					#[codec(index = 6)]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					InvalidResponder(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					InvalidResponderVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					AssetsTrapped(
						crate::networks::types::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					VersionChangeNotified(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 13)]
					SupportedVersionChanged(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					NotifyTargetSendFail(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::traits::Error,
					),
					#[codec(index = 15)]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 16)]
					InvalidQuerierVersion(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 17)]
					InvalidQuerier(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 18)]
					VersionNotifyStarted(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 19)]
					VersionNotifyRequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 20)]
					VersionNotifyUnrequested(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 21)]
					FeesPaid(
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::v3::multiasset::MultiAssets,
					),
					#[codec(index = 22)]
					AssetsClaimed(
						crate::networks::types::utils::H256,
						runtime_types::xcm::v3::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v3::multilocation::MultiLocation),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedMultiLocation,
						maybe_match_querier:
							::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedMultiLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct RemoteLockedFungibleRecord<_0> {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedMultiLocation,
					pub locker: runtime_types::xcm::VersionedMultiLocation,
					pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_0,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
		}
		pub mod pangolin_runtime {
			use super::runtime_types;
			pub mod pallets {
				use super::runtime_types;
				pub mod asset_manager {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum AssetType {
						#[codec(index = 0)]
						Xcm(runtime_types::xcm::v3::multilocation::MultiLocation),
					}
				}
				pub mod orml_xtokens {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum CurrencyId {
						#[codec(index = 0)]
						SelfReserve,
						#[codec(index = 1)]
						ForeignAsset(::core::primitive::u64),
					}
				}
				pub mod proxy {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum ProxyType {
						#[codec(index = 0)]
						Any,
						#[codec(index = 1)]
						NonTransfer,
						#[codec(index = 2)]
						Governance,
						#[codec(index = 3)]
						Staking,
						#[codec(index = 4)]
						IdentityJudgement,
						#[codec(index = 5)]
						CancelProxy,
						#[codec(index = 6)]
						EcdsaBridge,
					}
				}
				pub mod session {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct SessionKeys {
						pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					}
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct BridgeRejectObsoleteHeadersAndMessages;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<
						runtime_types::fp_account::AccountId20,
					>,
				),
				#[codec(index = 19)]
				Council(
					runtime_types::pallet_collective::RawOrigin<
						runtime_types::fp_account::AccountId20,
					>,
				),
				#[codec(index = 20)]
				TechnicalCommittee(
					runtime_types::pallet_collective::RawOrigin<
						runtime_types::fp_account::AccountId20,
					>,
				),
				#[codec(index = 33)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 34)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec(index = 44)]
				EthereumXcm(runtime_types::pallet_ethereum_xcm::RawOrigin),
				#[codec(index = 36)]
				Ethereum(runtime_types::pallet_ethereum::RawOrigin),
				#[codec(index = 38)]
				MessageTransact(runtime_types::darwinia_message_transact::LcmpEthOrigin),
				#[codec(index = 8)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 2)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 3)]
				ParachainInfo(runtime_types::parachain_info::pallet::Call),
				#[codec(index = 5)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 7)]
				Assets(runtime_types::pallet_assets::pallet::Call),
				#[codec(index = 9)]
				Deposit(runtime_types::darwinia_deposit::pallet::Call),
				#[codec(index = 10)]
				AccountMigration(runtime_types::darwinia_account_migration::pallet::Call),
				#[codec(index = 12)]
				DarwiniaStaking(runtime_types::darwinia_staking::pallet::Call),
				#[codec(index = 13)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 16)]
				MessageGadget(runtime_types::darwinia_message_gadget::pallet::Call),
				#[codec(index = 17)]
				EcdsaAuthority(runtime_types::darwinia_ecdsa_authority::pallet::Call),
				#[codec(index = 18)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 19)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 20)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Call2),
				#[codec(index = 21)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Call),
				#[codec(index = 22)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 23)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 24)]
				Tips(runtime_types::pallet_tips::pallet::Call),
				#[codec(index = 25)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 26)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 27)]
				Identity(runtime_types::pallet_identity::pallet::Call),
				#[codec(index = 28)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 29)]
				Preimage(runtime_types::pallet_preimage::pallet::Call),
				#[codec(index = 30)]
				Proxy(runtime_types::pallet_proxy::pallet::Call),
				#[codec(index = 32)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 33)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 34)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 44)]
				EthereumXcm(runtime_types::pallet_ethereum_xcm::pallet::Call),
				#[codec(index = 35)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 45)]
				AssetManager(runtime_types::pallet_asset_manager::pallet::Call),
				#[codec(index = 46)]
				XTokens(runtime_types::orml_xtokens::module::Call),
				#[codec(index = 47)]
				AssetLimit(runtime_types::darwinia_asset_limit::pallet::Call),
				#[codec(index = 36)]
				Ethereum(runtime_types::pallet_ethereum::pallet::Call),
				#[codec(index = 37)]
				EVM(runtime_types::pallet_evm::pallet::Call),
				#[codec(index = 38)]
				MessageTransact(runtime_types::darwinia_message_transact::pallet::Call),
				#[codec(index = 39)]
				BridgeMoonbaseGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Call),
				#[codec(index = 40)]
				BridgeMoonbaseParachain(runtime_types::pallet_bridge_parachains::pallet::Call),
				#[codec(index = 41)]
				BridgePangoroMessages(runtime_types::pallet_bridge_messages::pallet::Call),
				#[codec(index = 42)]
				BridgePangoroDispatch(runtime_types::pallet_bridge_dispatch::pallet::Call),
				#[codec(index = 43)]
				PangoroFeeMarket(runtime_types::pallet_fee_market::pallet::Call),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RuntimeError {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Error),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
				#[codec(index = 5)]
				Balances(runtime_types::pallet_balances::pallet::Error),
				#[codec(index = 7)]
				Assets(runtime_types::pallet_assets::pallet::Error),
				#[codec(index = 9)]
				Deposit(runtime_types::darwinia_deposit::pallet::Error),
				#[codec(index = 10)]
				AccountMigration(runtime_types::darwinia_account_migration::pallet::Error),
				#[codec(index = 12)]
				DarwiniaStaking(runtime_types::darwinia_staking::pallet::Error),
				#[codec(index = 13)]
				Session(runtime_types::pallet_session::pallet::Error),
				#[codec(index = 17)]
				EcdsaAuthority(runtime_types::darwinia_ecdsa_authority::pallet::Error),
				#[codec(index = 18)]
				Democracy(runtime_types::pallet_democracy::pallet::Error),
				#[codec(index = 19)]
				Council(runtime_types::pallet_collective::pallet::Error),
				#[codec(index = 20)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Error2),
				#[codec(index = 21)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Error),
				#[codec(index = 22)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Error),
				#[codec(index = 23)]
				Treasury(runtime_types::pallet_treasury::pallet::Error),
				#[codec(index = 24)]
				Tips(runtime_types::pallet_tips::pallet::Error),
				#[codec(index = 25)]
				Sudo(runtime_types::pallet_sudo::pallet::Error),
				#[codec(index = 26)]
				Utility(runtime_types::pallet_utility::pallet::Error),
				#[codec(index = 27)]
				Identity(runtime_types::pallet_identity::pallet::Error),
				#[codec(index = 28)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Error),
				#[codec(index = 29)]
				Preimage(runtime_types::pallet_preimage::pallet::Error),
				#[codec(index = 30)]
				Proxy(runtime_types::pallet_proxy::pallet::Error),
				#[codec(index = 32)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
				#[codec(index = 33)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
				#[codec(index = 34)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Error),
				#[codec(index = 44)]
				EthereumXcm(runtime_types::pallet_ethereum_xcm::pallet::Error),
				#[codec(index = 35)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Error),
				#[codec(index = 45)]
				AssetManager(runtime_types::pallet_asset_manager::pallet::Error),
				#[codec(index = 46)]
				XTokens(runtime_types::orml_xtokens::module::Error),
				#[codec(index = 47)]
				AssetLimit(runtime_types::darwinia_asset_limit::pallet::Error),
				#[codec(index = 36)]
				Ethereum(runtime_types::pallet_ethereum::pallet::Error),
				#[codec(index = 37)]
				EVM(runtime_types::pallet_evm::pallet::Error),
				#[codec(index = 38)]
				MessageTransact(runtime_types::darwinia_message_transact::pallet::Error),
				#[codec(index = 39)]
				BridgeMoonbaseGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Error),
				#[codec(index = 40)]
				BridgeMoonbaseParachain(runtime_types::pallet_bridge_parachains::pallet::Error),
				#[codec(index = 41)]
				BridgePangoroMessages(runtime_types::pallet_bridge_messages::pallet::Error),
				#[codec(index = 43)]
				PangoroFeeMarket(runtime_types::pallet_fee_market::pallet::Error),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 5)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 6)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 7)]
				Assets(runtime_types::pallet_assets::pallet::Event),
				#[codec(index = 9)]
				Deposit(runtime_types::darwinia_deposit::pallet::Event),
				#[codec(index = 10)]
				AccountMigration(runtime_types::darwinia_account_migration::pallet::Event),
				#[codec(index = 12)]
				DarwiniaStaking(runtime_types::darwinia_staking::pallet::Event),
				#[codec(index = 13)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 17)]
				EcdsaAuthority(runtime_types::darwinia_ecdsa_authority::pallet::Event),
				#[codec(index = 18)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 19)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 20)]
				TechnicalCommittee(runtime_types::pallet_collective::pallet::Event2),
				#[codec(index = 21)]
				PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Event),
				#[codec(index = 22)]
				TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 23)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 24)]
				Tips(runtime_types::pallet_tips::pallet::Event),
				#[codec(index = 25)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 26)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 27)]
				Identity(runtime_types::pallet_identity::pallet::Event),
				#[codec(index = 28)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 29)]
				Preimage(runtime_types::pallet_preimage::pallet::Event),
				#[codec(index = 30)]
				Proxy(runtime_types::pallet_proxy::pallet::Event),
				#[codec(index = 32)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 33)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 34)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 35)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 45)]
				AssetManager(runtime_types::pallet_asset_manager::pallet::Event),
				#[codec(index = 46)]
				XTokens(runtime_types::orml_xtokens::module::Event),
				#[codec(index = 47)]
				AssetLimit(runtime_types::darwinia_asset_limit::pallet::Event),
				#[codec(index = 36)]
				Ethereum(runtime_types::pallet_ethereum::pallet::Event),
				#[codec(index = 37)]
				EVM(runtime_types::pallet_evm::pallet::Event),
				#[codec(index = 40)]
				BridgeMoonbaseParachain(runtime_types::pallet_bridge_parachains::pallet::Event),
				#[codec(index = 41)]
				BridgePangoroMessages(runtime_types::pallet_bridge_messages::pallet::Event),
				#[codec(index = 42)]
				BridgePangoroDispatch(runtime_types::pallet_bridge_dispatch::pallet::Event),
				#[codec(index = 43)]
				PangoroFeeMarket(runtime_types::pallet_fee_market::pallet::Event),
			}
		}
		pub mod parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Call {}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum XcmpMessageFormat {
					#[codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v4 {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<crate::networks::types::utils::H256>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: ::core::primitive::u32,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct U256(pub [::core::primitive::u64; 4usize]);
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: CompactAs,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Permill(pub ::core::primitive::u32);
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: CompactAs,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum Void {}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct Header<_0, _1> {
						pub parent_hash: crate::networks::types::utils::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: crate::networks::types::utils::H256,
						pub extrinsics_root: crate::networks::types::utils::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct BlakeTwo256;
			}
			pub mod transaction_validity {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum InvalidTransaction {
					#[codec(index = 0)]
					Call,
					#[codec(index = 1)]
					Payment,
					#[codec(index = 2)]
					Future,
					#[codec(index = 3)]
					Stale,
					#[codec(index = 4)]
					BadProof,
					#[codec(index = 5)]
					AncientBirthBlock,
					#[codec(index = 6)]
					ExhaustsResources,
					#[codec(index = 7)]
					Custom(::core::primitive::u8),
					#[codec(index = 8)]
					BadMandatory,
					#[codec(index = 9)]
					MandatoryValidation,
					#[codec(index = 10)]
					BadSigner,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum TransactionValidityError {
					#[codec(index = 0)]
					Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
					#[codec(index = 1)]
					Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum UnknownTransaction {
					#[codec(index = 0)]
					CannotLookup,
					#[codec(index = 1)]
					NoUnsignedValidator,
					#[codec(index = 2)]
					Custom(::core::primitive::u8),
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub state_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct DoubleEncoded2 {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v2::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v2::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v2::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(
							runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v2::BodyId,
							part: runtime_types::xcm::v2::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v2::multiasset::AssetId,
						pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
					);
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v2::multiasset::AssetId,
							fun: runtime_types::xcm::v2::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v2::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
							runtime_types::xcm::v2::junction::Junction,
						),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v2::multilocation::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum BodyId {
					#[codec(index = 0)]
					Unit,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Index(#[codec(compact)] ::core::primitive::u32),
					#[codec(index = 3)]
					Executive,
					#[codec(index = 4)]
					Technical,
					#[codec(index = 5)]
					Legislative,
					#[codec(index = 6)]
					Judicial,
					#[codec(index = 7)]
					Defense,
					#[codec(index = 8)]
					Administration,
					#[codec(index = 9)]
					Treasury,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum BodyPart {
					#[codec(index = 0)]
					Voice,
					#[codec(index = 1)]
					Members {
						#[codec(compact)]
						count: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					Fraction {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					AtLeastProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					MoreThanProportion {
						#[codec(compact)]
						nom: ::core::primitive::u32,
						#[codec(compact)]
						denom: ::core::primitive::u32,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v2::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v2::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v2::multilocation::MultiLocation,
						assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v2::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v2::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum NetworkId {
					#[codec(index = 0)]
					Any,
					#[codec(index = 1)]
					Named(
						runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					Polkadot,
					#[codec(index = 3)]
					Kusama,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction2>);
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
					);
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
					#[derive(
						Debug,
						crate :: networks :: types :: ext :: codec :: Decode,
						crate :: networks :: types :: ext :: codec :: Encode,
						crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
						crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
					)]
					# [codec (crate = crate :: networks :: types :: ext :: codec)]
					#[decode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_decode"
					)]
					#[encode_as_type(
						crate_path = "crate :: networks :: types :: ext :: scale_encode"
					)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 1)]
						Incomplete(
							runtime_types::sp_weights::weight_v2::Weight,
							runtime_types::xcm::v3::traits::Error,
						),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v3::traits::Error),
					}
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v2::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedAssetId {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedMultiAsset {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAsset),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAsset),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedMultiAssets {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multiasset::MultiAssets),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedMultiLocation {
				#[codec(index = 1)]
				V2(runtime_types::xcm::v2::multilocation::MultiLocation),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multilocation::MultiLocation),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedResponse {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedXcm {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
			}
			#[derive(
				Debug,
				crate :: networks :: types :: ext :: codec :: Decode,
				crate :: networks :: types :: ext :: codec :: Encode,
				crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
				crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
			)]
			# [codec (crate = crate :: networks :: types :: ext :: codec)]
			#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
			#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
			pub enum VersionedXcm2 {
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm2),
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm2),
			}
		}
		pub mod xcm_primitives {
			use super::runtime_types;
			pub mod ethereum_xcm {
				use super::runtime_types;
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum EthereumXcmFee {
					#[codec(index = 0)]
					Manual(runtime_types::xcm_primitives::ethereum_xcm::ManualEthereumXcmFee),
					#[codec(index = 1)]
					Auto,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub enum EthereumXcmTransaction {
					#[codec(index = 0)]
					V1(runtime_types::xcm_primitives::ethereum_xcm::EthereumXcmTransactionV1),
					#[codec(index = 1)]
					V2(runtime_types::xcm_primitives::ethereum_xcm::EthereumXcmTransactionV2),
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct EthereumXcmTransactionV1 {
					pub gas_limit: runtime_types::primitive_types::U256,
					pub fee_payment: runtime_types::xcm_primitives::ethereum_xcm::EthereumXcmFee,
					pub action: runtime_types::ethereum::transaction::TransactionAction,
					pub value: runtime_types::primitive_types::U256,
					pub input: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub access_list: ::core::option::Option<
						::std::vec::Vec<(
							crate::networks::types::utils::H160,
							::std::vec::Vec<crate::networks::types::utils::H256>,
						)>,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct EthereumXcmTransactionV2 {
					pub gas_limit: runtime_types::primitive_types::U256,
					pub action: runtime_types::ethereum::transaction::TransactionAction,
					pub value: runtime_types::primitive_types::U256,
					pub input: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub access_list: ::core::option::Option<
						::std::vec::Vec<(
							crate::networks::types::utils::H160,
							::std::vec::Vec<crate::networks::types::utils::H256>,
						)>,
					>,
				}
				#[derive(
					Debug,
					crate :: networks :: types :: ext :: codec :: Decode,
					crate :: networks :: types :: ext :: codec :: Encode,
					crate :: networks :: types :: ext :: scale_decode :: DecodeAsType,
					crate :: networks :: types :: ext :: scale_encode :: EncodeAsType,
				)]
				# [codec (crate = crate :: networks :: types :: ext :: codec)]
				#[decode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_decode")]
				#[encode_as_type(crate_path = "crate :: networks :: types :: ext :: scale_encode")]
				pub struct ManualEthereumXcmFee {
					pub gas_price: ::core::option::Option<runtime_types::primitive_types::U256>,
					pub max_fee_per_gas:
						::core::option::Option<runtime_types::primitive_types::U256>,
				}
			}
		}
	}
}
