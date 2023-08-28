//! Integration test for `RR-0140` (basic).
//! Ring scan lattice modules implement pipeline v80 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0140_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8f, 0x91];
    let first = relayring::capabilities::rr_0140_ring_scan_lattice_module::evaluate(fixture).expect("RR-0140: Ring scan lattice modules implement pipeline v80");
    let second = relayring::capabilities::rr_0140_ring_scan_lattice_module::evaluate(fixture).expect("RR-0140: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0140: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0140: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
