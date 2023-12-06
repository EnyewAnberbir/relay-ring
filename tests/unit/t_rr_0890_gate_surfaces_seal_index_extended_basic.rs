//! Integration test for `RR-0890` (basic).
//! Extended: Gate surfaces seal index validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0890_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x85];
    let first = relayring::capabilities::rr_0890_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0890: Extended: Gate surfaces seal index validate resolver v25");
    let second = relayring::capabilities::rr_0890_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0890: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0890: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0890: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
