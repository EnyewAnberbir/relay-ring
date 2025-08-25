//! Integration test for `RR-0431` (stream).
//! Gate gateway ack replay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0431_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let direct = relayring::capabilities::rr_0431_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0431: direct Gate gateway ack replay export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0431_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0431: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0431: stream path must consume input");
}
