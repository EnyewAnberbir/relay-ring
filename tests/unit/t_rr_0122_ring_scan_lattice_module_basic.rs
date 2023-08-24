//! Integration test for `RR-0122` (basic).
//! Ring scan lattice modules harden index v62 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0122_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x7f];
    let first = relayring::capabilities::rr_0122_ring_scan_lattice_module::evaluate(fixture).expect("RR-0122: Ring scan lattice modules harden index v62");
    let second = relayring::capabilities::rr_0122_ring_scan_lattice_module::evaluate(fixture).expect("RR-0122: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0122: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0122: scanner should emit domain hints");
}
