//! Integration test for `RR-0587` (basic).
//! Extended: Ring scan lattice modules integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0587_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let first = relayring::capabilities::rr_0587_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0587: Extended: Ring scan lattice modules integrate validator v27");
    let second = relayring::capabilities::rr_0587_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0587: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0587: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0587: scanner should emit domain hints");
}
