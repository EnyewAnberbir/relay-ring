//! Integration test for `RR-0634` (basic).
//! Extended: Ring scan lattice modules optimize registry v74 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0634_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x81, 0x83];
    let first = relayring::capabilities::rr_0634_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0634: Extended: Ring scan lattice modules optimize registry v74");
    let second = relayring::capabilities::rr_0634_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0634: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0634: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0634: scanner should emit domain hints");
}
