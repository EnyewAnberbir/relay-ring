//! Integration test for `RR-0952` (stream).
//! Extended: Gate gateway ack replay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0952_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let direct = relayring::capabilities::rr_0952_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0952: direct Extended: Gate gateway ack replay integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0952_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0952: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0952: stream path must consume input");
}
