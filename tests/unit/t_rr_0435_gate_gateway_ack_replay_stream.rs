//! Integration test for `RR-0435` (stream).
//! Gate gateway ack replay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0435_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let direct = relayring::capabilities::rr_0435_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0435: direct Gate gateway ack replay implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0435_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0435: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0435: stream path must consume input");
}
