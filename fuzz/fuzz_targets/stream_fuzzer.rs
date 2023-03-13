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
    sink ^= ring_append::surface_ring_append_append_export_batch(input).digest;
    sink ^= ring_append::surface_ring_append_segment_replay_scan(input).digest;
    sink ^= journal_seek::surface_journal_seek_sink_journal_seek(input).digest;
    sink ^= segment_seal::surface_segment_seal_export_compact_pass(input).digest;
    sink ^= export_batch::surface_export_batch_batch_agent_ack(input).digest;
    sink ^= offset_index::surface_offset_index_append_export_batch(input).digest;
    sink ^= offset_index::surface_offset_index_segment_replay_scan(input).digest;
    sink ^= compact_pass::surface_compact_pass_sink_journal_seek(input).digest;
    sink ^= gateway_push::surface_gateway_push_export_compact_pass(input).digest;
    sink ^= replay_scan::surface_replay_scan_batch_agent_ack(input).digest;
    sink ^= checksum_lane::surface_checksum_lane_append_export_batch(input).digest;
    sink ^= checksum_lane::surface_checksum_lane_segment_replay_scan(input).digest;
    sink ^= agent_ack::surface_agent_ack_sink_journal_seek(input).digest;
    std::hint::black_box(sink);
});
