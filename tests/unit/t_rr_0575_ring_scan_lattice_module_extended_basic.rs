//! Integration test for `RR-0575` (basic).
//! Extended: Ring scan lattice modules validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0575_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let first = relayring::capabilities::rr_0575_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0575: Extended: Ring scan lattice modules validate resolver v15");
    let second = relayring::capabilities::rr_0575_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0575: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0575: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0575: scanner should emit domain hints");
}
