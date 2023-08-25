//! Integration test for `RR-0126` (basic).
//! Ring scan lattice modules export adapter v66 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0126_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81, 0x83];
    let first = relayring::capabilities::rr_0126_ring_scan_lattice_module::evaluate(fixture).expect("RR-0126: Ring scan lattice modules export adapter v66");
    let second = relayring::capabilities::rr_0126_ring_scan_lattice_module::evaluate(fixture).expect("RR-0126: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0126: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0126: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
