//! Integration test for `RR-0590` (basic).
//! Extended: Ring scan lattice modules implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0590_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x57];
    let first = relayring::capabilities::rr_0590_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0590: Extended: Ring scan lattice modules implement pipeline v30");
    let second = relayring::capabilities::rr_0590_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0590: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0590: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0590: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
