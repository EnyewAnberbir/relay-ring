//! Integration test for `RR-0143` (basic).
//! Ring scan lattice modules wire planner v83 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0143_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x92, 0x94];
    let first = relayring::capabilities::rr_0143_ring_scan_lattice_module::evaluate(fixture).expect("RR-0143: Ring scan lattice modules wire planner v83");
    let second = relayring::capabilities::rr_0143_ring_scan_lattice_module::evaluate(fixture).expect("RR-0143: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0143: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0143: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
