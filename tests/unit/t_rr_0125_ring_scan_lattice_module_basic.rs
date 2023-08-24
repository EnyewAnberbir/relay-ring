//! Integration test for `RR-0125` (basic).
//! Ring scan lattice modules validate resolver v65 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0125_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let first = relayring::capabilities::rr_0125_ring_scan_lattice_module::evaluate(fixture).expect("RR-0125: Ring scan lattice modules validate resolver v65");
    let second = relayring::capabilities::rr_0125_ring_scan_lattice_module::evaluate(fixture).expect("RR-0125: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0125: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0125: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
