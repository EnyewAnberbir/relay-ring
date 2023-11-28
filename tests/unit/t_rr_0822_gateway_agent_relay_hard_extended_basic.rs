//! Integration test for `RR-0822` (basic).
//! Extended: Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0822_gateway_agent_relay_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let first = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0822: Extended: Gateway agent relay harden index v22");
    let second = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0822: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0822: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0822: scanner should emit domain hints");
}
