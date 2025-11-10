//! Integration test for `RR-0949` (stream).
//! Extended: Gate gateway ack replay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0949_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let direct = relayring::capabilities::rr_0949_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0949: direct Extended: Gate gateway ack replay optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0949_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0949: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0949: stream path must consume input");
}
