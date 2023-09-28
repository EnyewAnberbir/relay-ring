//! Integration test for `RR-0370` (basic).
//! Gate surfaces seal index validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0370_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x79];
    let first = relayring::capabilities::rr_0370_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0370: Gate surfaces seal index validate resolver v5");
    let second = relayring::capabilities::rr_0370_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0370: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0370: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0370: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
