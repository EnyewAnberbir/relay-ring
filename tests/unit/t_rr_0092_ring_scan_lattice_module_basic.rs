//! Integration test for `RR-0092` (basic).
//! Ring scan lattice modules harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0092_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let first = relayring::capabilities::rr_0092_ring_scan_lattice_module::evaluate(fixture).expect("RR-0092: Ring scan lattice modules harden index v32");
    let second = relayring::capabilities::rr_0092_ring_scan_lattice_module::evaluate(fixture).expect("RR-0092: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0092: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0092: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
