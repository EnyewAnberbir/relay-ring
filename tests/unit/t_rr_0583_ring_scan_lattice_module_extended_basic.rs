//! Integration test for `RR-0583` (basic).
//! Extended: Ring scan lattice modules wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0583_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let first = relayring::capabilities::rr_0583_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0583: Extended: Ring scan lattice modules wire planner v23");
    let second = relayring::capabilities::rr_0583_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0583: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0583: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0583: scanner should emit domain hints");
}
