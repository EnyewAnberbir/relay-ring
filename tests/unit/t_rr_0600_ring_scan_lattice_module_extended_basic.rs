//! Integration test for `RR-0600` (basic).
//! Extended: Ring scan lattice modules implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0600_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let first = relayring::capabilities::rr_0600_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0600: Extended: Ring scan lattice modules implement pipeline v40");
    let second = relayring::capabilities::rr_0600_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0600: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0600: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0600: stats visits every byte");
}
