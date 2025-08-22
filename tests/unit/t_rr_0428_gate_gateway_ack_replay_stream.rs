//! Integration test for `RR-0428` (stream).
//! Gate gateway ack replay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0428_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let direct = relayring::capabilities::rr_0428_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0428: direct Gate gateway ack replay wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0428_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0428: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0428: stream path must consume input");
}
