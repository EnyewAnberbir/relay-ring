//! Integration test for `RR-0946` (basic).
//! Extended: Gate gateway ack replay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0946_gate_gateway_ack_replay_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let first = relayring::capabilities::rr_0946_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0946: Extended: Gate gateway ack replay extend codec v21");
    let second = relayring::capabilities::rr_0946_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0946: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0946: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0946: scanner should emit domain hints");
}
