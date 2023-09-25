//! Integration test for `RR-0340` (basic).
//! Gate surfaces append seek validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0340_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x5b];
    let first = relayring::capabilities::rr_0340_gate_surfaces_append_see::evaluate(fixture).expect("RR-0340: Gate surfaces append seek validate resolver v5");
    let second = relayring::capabilities::rr_0340_gate_surfaces_append_see::evaluate(fixture).expect("RR-0340: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0340: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0340: scanner should emit domain hints");
}
