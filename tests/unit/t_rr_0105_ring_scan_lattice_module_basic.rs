//! Integration test for `RR-0105` (basic).
//! Ring scan lattice modules validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0105_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let first = relayring::capabilities::rr_0105_ring_scan_lattice_module::evaluate(fixture).expect("RR-0105: Ring scan lattice modules validate resolver v45");
    let second = relayring::capabilities::rr_0105_ring_scan_lattice_module::evaluate(fixture).expect("RR-0105: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0105: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0105: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
