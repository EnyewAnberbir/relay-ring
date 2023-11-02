//! Integration test for `RR-0631` (basic).
//! Extended: Ring scan lattice modules extend codec v71 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0631_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x80];
    let first = relayring::capabilities::rr_0631_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0631: Extended: Ring scan lattice modules extend codec v71");
    let second = relayring::capabilities::rr_0631_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0631: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0631: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0631: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
