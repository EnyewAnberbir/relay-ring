//! Integration test for `RR-0605` (basic).
//! Extended: Ring scan lattice modules validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0605_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x66];
    let first = relayring::capabilities::rr_0605_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0605: Extended: Ring scan lattice modules validate resolver v45");
    let second = relayring::capabilities::rr_0605_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0605: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0605: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0605: scanner should emit domain hints");
}
