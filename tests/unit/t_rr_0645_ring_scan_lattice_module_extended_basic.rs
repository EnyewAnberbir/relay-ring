//! Integration test for `RR-0645` (basic).
//! Extended: Ring scan lattice modules validate resolver v85 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0645_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x8e];
    let first = relayring::capabilities::rr_0645_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0645: Extended: Ring scan lattice modules validate resolver v85");
    let second = relayring::capabilities::rr_0645_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0645: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0645: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0645: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
