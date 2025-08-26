//! Integration test for `RR-0440` (stream).
//! Gate gateway ack replay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0440_gate_gateway_ack_replay_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbd, 0xbf];
    let direct = relayring::capabilities::rr_0440_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0440: direct Gate gateway ack replay validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0440_gate_gateway_ack_replay::evaluate(&copied).expect("RR-0440: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0440: stream path must consume input");
}
