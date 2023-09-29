//! Integration test for `RR-0378` (basic).
//! Gate surfaces seal index wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0378_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let first = relayring::capabilities::rr_0378_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0378: Gate surfaces seal index wire planner v13");
    let second = relayring::capabilities::rr_0378_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0378: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0378: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0378: scanner should emit domain hints");
}
