//! Integration test for `RR-0082` (basic).
//! Ring scan lattice modules harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0082_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0x57];
    let first = relayring::capabilities::rr_0082_ring_scan_lattice_module::evaluate(fixture).expect("RR-0082: Ring scan lattice modules harden index v22");
    let second = relayring::capabilities::rr_0082_ring_scan_lattice_module::evaluate(fixture).expect("RR-0082: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0082: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0082: scanner should emit domain hints");
}
