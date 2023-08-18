//! Integration test for `RR-0083` (basic).
//! Ring scan lattice modules wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0083_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x58];
    let first = relayring::capabilities::rr_0083_ring_scan_lattice_module::evaluate(fixture).expect("RR-0083: Ring scan lattice modules wire planner v23");
    let second = relayring::capabilities::rr_0083_ring_scan_lattice_module::evaluate(fixture).expect("RR-0083: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0083: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0083: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
