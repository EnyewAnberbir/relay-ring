//! Integration test for `RR-0608` (basic).
//! Extended: Ring scan lattice modules refactor mutator v48 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0608_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x69];
    let first = relayring::capabilities::rr_0608_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0608: Extended: Ring scan lattice modules refactor mutator v48");
    let second = relayring::capabilities::rr_0608_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0608: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0608: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0608: scanner should emit domain hints");
}
