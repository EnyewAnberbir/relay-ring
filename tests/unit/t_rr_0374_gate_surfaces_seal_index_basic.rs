//! Integration test for `RR-0374` (basic).
//! Gate surfaces seal index benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0374_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let first = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0374: Gate surfaces seal index benchmark reporter v9");
    let second = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0374: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0374: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0374: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
