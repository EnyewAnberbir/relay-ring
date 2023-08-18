//! Integration test for `RR-0077` (basic).
//! Ring scan lattice modules integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0077_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let first = relayring::capabilities::rr_0077_ring_scan_lattice_module::evaluate(fixture).expect("RR-0077: Ring scan lattice modules integrate validator v17");
    let second = relayring::capabilities::rr_0077_ring_scan_lattice_module::evaluate(fixture).expect("RR-0077: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0077: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0077: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
