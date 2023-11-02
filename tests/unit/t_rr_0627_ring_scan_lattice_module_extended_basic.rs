//! Integration test for `RR-0627` (basic).
//! Extended: Ring scan lattice modules integrate validator v67 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0627_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x7c];
    let first = relayring::capabilities::rr_0627_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0627: Extended: Ring scan lattice modules integrate validator v67");
    let second = relayring::capabilities::rr_0627_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0627: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0627: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0627: scanner should emit domain hints");
}
