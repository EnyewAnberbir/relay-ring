//! Integration test for `RR-0429` (stream).
//! Gate gateway ack replay optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0429_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let direct = relayring::capabilities::rr_0429_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0429: direct Gate gateway ack replay optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0429_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0429: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0429: stream path must consume input");
}
