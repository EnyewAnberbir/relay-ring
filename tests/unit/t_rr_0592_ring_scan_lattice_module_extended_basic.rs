//! Integration test for `RR-0592` (basic).
//! Extended: Ring scan lattice modules harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0592_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x59];
    let first = relayring::capabilities::rr_0592_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0592: Extended: Ring scan lattice modules harden index v32");
    let second = relayring::capabilities::rr_0592_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0592: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0592: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0592: scanner should emit domain hints");
}
