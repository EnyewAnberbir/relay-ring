//! Integration test for `RR-0432` (basic).
//! Gate gateway ack replay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0432_gate_gateway_ack_replay_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let first = relayring::capabilities::rr_0432_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0432: Gate gateway ack replay integrate validator v7");
    let second = relayring::capabilities::rr_0432_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0432: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0432: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0432: scanner should emit domain hints");
}
