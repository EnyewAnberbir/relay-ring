//! Integration test for `RR-0621` (basic).
//! Extended: Ring scan lattice modules extend codec v61 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0621_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x76];
    let first = relayring::capabilities::rr_0621_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0621: Extended: Ring scan lattice modules extend codec v61");
    let second = relayring::capabilities::rr_0621_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0621: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0621: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0621: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
