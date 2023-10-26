//! Integration test for `RR-0582` (basic).
//! Extended: Ring scan lattice modules harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0582_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let first = relayring::capabilities::rr_0582_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0582: Extended: Ring scan lattice modules harden index v22");
    let second = relayring::capabilities::rr_0582_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0582: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0582: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0582: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
