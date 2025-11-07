//! Integration test for `RR-0937` (stream).
//! Extended: Gate gateway ack replay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0937_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let direct = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0937: direct Extended: Gate gateway ack replay harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0937_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0937: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0937: stream path must consume input");
}
