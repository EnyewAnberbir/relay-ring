//! Integration test for `RR-0943` (stream).
//! Extended: Gate gateway ack replay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0943_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let direct = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0943: direct Extended: Gate gateway ack replay refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0943: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0943: stream path must consume input");
}
