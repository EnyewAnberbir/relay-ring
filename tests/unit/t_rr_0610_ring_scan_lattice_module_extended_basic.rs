//! Integration test for `RR-0610` (basic).
//! Extended: Ring scan lattice modules implement pipeline v50 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0610_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let first = relayring::capabilities::rr_0610_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0610: Extended: Ring scan lattice modules implement pipeline v50");
    let second = relayring::capabilities::rr_0610_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0610: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0610: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0610: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
