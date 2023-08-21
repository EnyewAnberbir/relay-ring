//! Integration test for `RR-0090` (basic).
//! Ring scan lattice modules implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0090_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0x5f];
    let first = relayring::capabilities::rr_0090_ring_scan_lattice_module::evaluate(fixture).expect("RR-0090: Ring scan lattice modules implement pipeline v30");
    let second = relayring::capabilities::rr_0090_ring_scan_lattice_module::evaluate(fixture).expect("RR-0090: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0090: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0090: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
