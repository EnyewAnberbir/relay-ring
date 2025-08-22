//! Integration test for `RR-0427` (stream).
//! Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0427_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let direct = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0427: direct Gate gateway ack replay harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0427: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0427: stream path must consume input");
}
