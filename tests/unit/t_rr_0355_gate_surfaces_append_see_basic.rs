//! Integration test for `RR-0355` (basic).
//! Gate surfaces append seek implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0355_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x6a];
    let first = relayring::capabilities::rr_0355_gate_surfaces_append_see::evaluate(fixture).expect("RR-0355: Gate surfaces append seek implement pipeline v20");
    let second = relayring::capabilities::rr_0355_gate_surfaces_append_see::evaluate(fixture).expect("RR-0355: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0355: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0355: stats visits every byte");
}
