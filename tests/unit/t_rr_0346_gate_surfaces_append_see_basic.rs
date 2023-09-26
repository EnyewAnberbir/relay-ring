//! Integration test for `RR-0346` (basic).
//! Gate surfaces append seek extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0346_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let first = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(fixture).expect("RR-0346: Gate surfaces append seek extend codec v11");
    let second = relayring::capabilities::rr_0346_gate_surfaces_append_see::evaluate(fixture).expect("RR-0346: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0346: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0346: stats visits every byte");
}
