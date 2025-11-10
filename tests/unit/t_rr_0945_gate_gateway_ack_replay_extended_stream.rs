//! Integration test for `RR-0945` (stream).
//! Extended: Gate gateway ack replay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0945_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let direct = relayring::capabilities::rr_0945_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0945: direct Extended: Gate gateway ack replay implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0945_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0945: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0945: stream path must consume input");
}
