//! Integration test for `RR-0123` (basic).
//! Ring scan lattice modules wire planner v63 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0123_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x80];
    let first = relayring::capabilities::rr_0123_ring_scan_lattice_module::evaluate(fixture).expect("RR-0123: Ring scan lattice modules wire planner v63");
    let second = relayring::capabilities::rr_0123_ring_scan_lattice_module::evaluate(fixture).expect("RR-0123: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0123: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0123: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
