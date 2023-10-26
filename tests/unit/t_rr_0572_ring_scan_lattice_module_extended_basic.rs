//! Integration test for `RR-0572` (basic).
//! Extended: Ring scan lattice modules harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0572_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let first = relayring::capabilities::rr_0572_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0572: Extended: Ring scan lattice modules harden index v12");
    let second = relayring::capabilities::rr_0572_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0572: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0572: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0572: scanner should emit domain hints");
}
