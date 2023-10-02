//! Integration test for `RR-0388` (basic).
//! Gate surfaces seal index wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0388_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let first = relayring::capabilities::rr_0388_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0388: Gate surfaces seal index wire planner v23");
    let second = relayring::capabilities::rr_0388_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0388: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0388: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0388: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
