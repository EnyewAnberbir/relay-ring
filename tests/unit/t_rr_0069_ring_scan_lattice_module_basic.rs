//! Integration test for `RR-0069` (basic).
//! Ring scan lattice modules benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0069_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x4a];
    let first = relayring::capabilities::rr_0069_ring_scan_lattice_module::evaluate(fixture).expect("RR-0069: Ring scan lattice modules benchmark reporter v9");
    let second = relayring::capabilities::rr_0069_ring_scan_lattice_module::evaluate(fixture).expect("RR-0069: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0069: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0069: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
