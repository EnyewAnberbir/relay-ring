//! Integration test for `RR-0095` (basic).
//! Ring scan lattice modules validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0095_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let first = relayring::capabilities::rr_0095_ring_scan_lattice_module::evaluate(fixture).expect("RR-0095: Ring scan lattice modules validate resolver v35");
    let second = relayring::capabilities::rr_0095_ring_scan_lattice_module::evaluate(fixture).expect("RR-0095: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0095: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0095: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
