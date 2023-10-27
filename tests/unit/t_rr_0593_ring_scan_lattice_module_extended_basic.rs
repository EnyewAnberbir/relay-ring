//! Integration test for `RR-0593` (basic).
//! Extended: Ring scan lattice modules wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0593_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let first = relayring::capabilities::rr_0593_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0593: Extended: Ring scan lattice modules wire planner v33");
    let second = relayring::capabilities::rr_0593_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0593: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0593: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0593: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
