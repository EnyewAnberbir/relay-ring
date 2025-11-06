//! Integration test for `RR-0926` (stream).
//! Extended: Gate gateway ack replay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0926_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa7, 0xa9];
    let direct = relayring::capabilities::rr_0926_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0926: direct Extended: Gate gateway ack replay extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0926_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0926: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0926: stream path must consume input");
}
