//! Integration test for `RR-0955` (basic).
//! Extended: Gate gateway ack replay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0955_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let first = relayring::capabilities::rr_0955_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0955: Extended: Gate gateway ack replay implement pipeline v30");
    let second = relayring::capabilities::rr_0955_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0955: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0955: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0955: scanner should emit domain hints");
}
