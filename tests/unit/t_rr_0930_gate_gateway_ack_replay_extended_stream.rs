//! Integration test for `RR-0930` (stream).
//! Extended: Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0930_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab, 0xad];
    let direct = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0930: direct Extended: Gate gateway ack replay validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0930: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0930: stream path must consume input");
}
