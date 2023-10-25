//! Integration test for `RR-0568` (basic).
//! Extended: Ring scan lattice modules refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0568_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let first = relayring::capabilities::rr_0568_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0568: Extended: Ring scan lattice modules refactor mutator v8");
    let second = relayring::capabilities::rr_0568_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0568: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0568: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0568: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
