//! Integration test for `RR-0392` (basic).
//! Gate surfaces seal index integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0392_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8d, 0x8f];
    let first = relayring::capabilities::rr_0392_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0392: Gate surfaces seal index integrate validator v27");
    let second = relayring::capabilities::rr_0392_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0392: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0392: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0392: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
