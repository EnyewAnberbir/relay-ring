//! Integration test for `RR-0455` (basic).
//! Gate gateway ack replay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0455_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let first = relayring::capabilities::rr_0455_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0455: Gate gateway ack replay implement pipeline v30");
    let second = relayring::capabilities::rr_0455_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0455: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0455: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0455: stats visits every byte");
}
