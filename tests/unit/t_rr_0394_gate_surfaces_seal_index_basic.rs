//! Integration test for `RR-0394` (basic).
//! Gate surfaces seal index benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0394_gate_surfaces_seal_index_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8f, 0x91];
    let first = relayring::capabilities::rr_0394_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0394: Gate surfaces seal index benchmark reporter v29");
    let second = relayring::capabilities::rr_0394_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0394: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0394: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0394: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
