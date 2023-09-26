//! Integration test for `RR-0342` (basic).
//! Gate surfaces append seek integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0342_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x5d];
    let first = relayring::capabilities::rr_0342_gate_surfaces_append_see::evaluate(fixture).expect("RR-0342: Gate surfaces append seek integrate validator v7");
    let second = relayring::capabilities::rr_0342_gate_surfaces_append_see::evaluate(fixture).expect("RR-0342: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0342: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0342: stats visits every byte");
}
