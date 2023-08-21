//! Integration test for `RR-0087` (basic).
//! Ring scan lattice modules integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0087_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let first = relayring::capabilities::rr_0087_ring_scan_lattice_module::evaluate(fixture).expect("RR-0087: Ring scan lattice modules integrate validator v27");
    let second = relayring::capabilities::rr_0087_ring_scan_lattice_module::evaluate(fixture).expect("RR-0087: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0087: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0087: scanner should emit domain hints");
}
