//! Integration test for `RR-0566` (basic).
//! Extended: Ring scan lattice modules export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0566_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let first = relayring::capabilities::rr_0566_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0566: Extended: Ring scan lattice modules export adapter v6");
    let second = relayring::capabilities::rr_0566_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0566: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0566: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0566: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
