//! Integration test for `RR-0454` (stream).
//! Gate gateway ack replay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0454_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let direct = relayring::capabilities::rr_0454_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0454: direct Gate gateway ack replay benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0454_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0454: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0454: stream path must consume input");
}
