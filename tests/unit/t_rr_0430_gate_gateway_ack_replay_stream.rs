//! Integration test for `RR-0430` (stream).
//! Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0430_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let direct = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0430: direct Gate gateway ack replay validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0430: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0430: stream path must consume input");
}
