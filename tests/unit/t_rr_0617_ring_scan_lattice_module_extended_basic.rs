//! Integration test for `RR-0617` (basic).
//! Extended: Ring scan lattice modules integrate validator v57 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0617_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x72];
    let first = relayring::capabilities::rr_0617_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0617: Extended: Ring scan lattice modules integrate validator v57");
    let second = relayring::capabilities::rr_0617_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0617: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0617: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0617: scanner should emit domain hints");
}
