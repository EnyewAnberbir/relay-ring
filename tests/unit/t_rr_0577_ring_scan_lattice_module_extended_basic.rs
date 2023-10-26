//! Integration test for `RR-0577` (basic).
//! Extended: Ring scan lattice modules integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0577_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x4a];
    let first = relayring::capabilities::rr_0577_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0577: Extended: Ring scan lattice modules integrate validator v17");
    let second = relayring::capabilities::rr_0577_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0577: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0577: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0577: scanner should emit domain hints");
}
