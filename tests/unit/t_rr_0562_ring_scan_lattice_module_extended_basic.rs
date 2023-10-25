//! Integration test for `RR-0562` (basic).
//! Extended: Ring scan lattice modules harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0562_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let first = relayring::capabilities::rr_0562_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0562: Extended: Ring scan lattice modules harden index v2");
    let second = relayring::capabilities::rr_0562_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0562: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0562: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0562: stats visits every byte");
}
