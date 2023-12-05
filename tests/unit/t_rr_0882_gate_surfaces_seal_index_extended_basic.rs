//! Integration test for `RR-0882` (basic).
//! Extended: Gate surfaces seal index integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0882_gate_surfaces_seal_index_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let first = relayring::capabilities::rr_0882_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0882: Extended: Gate surfaces seal index integrate validator v17");
    let second = relayring::capabilities::rr_0882_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0882: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0882: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0882: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
