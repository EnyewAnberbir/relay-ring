//! Integration test for `RR-0130` (basic).
//! Ring scan lattice modules implement pipeline v70 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0130_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let first = relayring::capabilities::rr_0130_ring_scan_lattice_module::evaluate(fixture).expect("RR-0130: Ring scan lattice modules implement pipeline v70");
    let second = relayring::capabilities::rr_0130_ring_scan_lattice_module::evaluate(fixture).expect("RR-0130: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0130: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0130: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
