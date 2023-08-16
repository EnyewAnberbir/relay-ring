//! Integration test for `RR-0065` (basic).
//! Ring scan lattice modules validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0065_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let first = relayring::capabilities::rr_0065_ring_scan_lattice_module::evaluate(fixture).expect("RR-0065: Ring scan lattice modules validate resolver v5");
    let second = relayring::capabilities::rr_0065_ring_scan_lattice_module::evaluate(fixture).expect("RR-0065: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0065: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0065: scanner should emit domain hints");
}
