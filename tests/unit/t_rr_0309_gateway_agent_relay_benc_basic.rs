//! Integration test for `RR-0309` (basic).
//! Gateway agent relay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0309_gateway_agent_relay_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let first = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0309: Gateway agent relay benchmark reporter v9");
    let second = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0309: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0309: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0309: stats visits every byte");
}
