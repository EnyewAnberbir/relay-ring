//! Integration test for `RR-0345` (basic).
//! Gate surfaces append seek implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0345_gate_surfaces_append_see_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e, 0x60];
    let first = relayring::capabilities::rr_0345_gate_surfaces_append_see::evaluate(fixture).expect("RR-0345: Gate surfaces append seek implement pipeline v10");
    let second = relayring::capabilities::rr_0345_gate_surfaces_append_see::evaluate(fixture).expect("RR-0345: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0345: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0345: scanner should emit domain hints");
}
