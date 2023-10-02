//! Integration test for `RR-0387` (basic).
//! Gate surfaces seal index harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0387_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let first = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0387: Gate surfaces seal index harden index v22");
    let second = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0387: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0387: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0387: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
