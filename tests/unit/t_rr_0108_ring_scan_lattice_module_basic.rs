//! Integration test for `RR-0108` (basic).
//! Ring scan lattice modules refactor mutator v48 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0108_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let first = relayring::capabilities::rr_0108_ring_scan_lattice_module::evaluate(fixture).expect("RR-0108: Ring scan lattice modules refactor mutator v48");
    let second = relayring::capabilities::rr_0108_ring_scan_lattice_module::evaluate(fixture).expect("RR-0108: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0108: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0108: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
