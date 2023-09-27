//! Integration test for `RR-0352` (basic).
//! Gate surfaces append seek integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0352_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x67];
    let first = relayring::capabilities::rr_0352_gate_surfaces_append_see::evaluate(fixture).expect("RR-0352: Gate surfaces append seek integrate validator v17");
    let second = relayring::capabilities::rr_0352_gate_surfaces_append_see::evaluate(fixture).expect("RR-0352: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0352: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0352: stats visits every byte");
}
