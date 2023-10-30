//! Integration test for `RR-0603` (basic).
//! Extended: Ring scan lattice modules wire planner v43 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0603_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let first = relayring::capabilities::rr_0603_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0603: Extended: Ring scan lattice modules wire planner v43");
    let second = relayring::capabilities::rr_0603_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0603: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0603: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0603: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
