#![no_main]

use libfuzzer_sys::fuzz_target;
use relayring::wire::decode;
use relayring::wire::validate;
use relayring::telemetry::journal_report;
use relayring::runtime::sequencer;
use relayring::gates::agent_ack;
use relayring::gates::checksum_lane;
use relayring::gates::compact_pass;
use relayring::gates::export_batch;
use relayring::gates::gateway_push;
use relayring::gates::journal_seek;
use relayring::gates::offset_index;
use relayring::gates::replay_scan;
use relayring::gates::ring_append;
use relayring::gates::segment_seal;

fuzz_target!(|input: &[u8]| {
    if input.len() > 400_000 { return; }
    let _ = decode::decode(input);
    let _ = validate::validate_frame(input, false);
    let _ = journal_report::run(input);
    let _ = sequencer::engine_sequencer_run_pipeline(input, false);
    let mut sink = 0u64;
    sink ^= ring_append::surface_ring_append_index_offset_index(input).digest;
    sink ^= journal_seek::surface_journal_seek_compact_checksum_lane(input).digest;
    sink ^= journal_seek::surface_journal_seek_agent_segment_seal(input).digest;
    sink ^= segment_seal::surface_segment_seal_relay_gateway_push(input).digest;
    sink ^= export_batch::surface_export_batch_offset_ring_append(input).digest;
    sink ^= offset_index::surface_offset_index_index_offset_index(input).digest;
    sink ^= compact_pass::surface_compact_pass_compact_checksum_lane(input).digest;
    sink ^= compact_pass::surface_compact_pass_agent_segment_seal(input).digest;
    sink ^= gateway_push::surface_gateway_push_relay_gateway_push(input).digest;
    sink ^= replay_scan::surface_replay_scan_offset_ring_append(input).digest;
    sink ^= checksum_lane::surface_checksum_lane_index_offset_index(input).digest;
    sink ^= agent_ack::surface_agent_ack_compact_checksum_lane(input).digest;
    sink ^= agent_ack::surface_agent_ack_agent_segment_seal(input).digest;
    std::hint::black_box(sink);
});
