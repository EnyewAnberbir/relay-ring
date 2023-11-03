//! Integration test for `RR-0642` (basic).
//! Extended: Ring scan lattice modules harden index v82 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0642_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let first = relayring::capabilities::rr_0642_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0642: Extended: Ring scan lattice modules harden index v82");
    let second = relayring::capabilities::rr_0642_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0642: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0642: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0642: scanner should emit domain hints");
}
