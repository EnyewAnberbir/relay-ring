//! Integration test for `RR-0632` (basic).
//! Extended: Ring scan lattice modules harden index v72 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0632_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let first = relayring::capabilities::rr_0632_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0632: Extended: Ring scan lattice modules harden index v72");
    let second = relayring::capabilities::rr_0632_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0632: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0632: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0632: scanner should emit domain hints");
}
