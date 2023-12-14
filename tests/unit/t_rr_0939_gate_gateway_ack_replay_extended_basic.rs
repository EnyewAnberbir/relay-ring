//! Integration test for `RR-0939` (basic).
//! Extended: Gate gateway ack replay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0939_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let first = relayring::capabilities::rr_0939_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0939: Extended: Gate gateway ack replay optimize registry v14");
    let second = relayring::capabilities::rr_0939_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0939: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0939: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0939: scanner should emit domain hints");
}
