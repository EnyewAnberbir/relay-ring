//! Integration test for `RR-0574` (basic).
//! Extended: Ring scan lattice modules optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0574_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let first = relayring::capabilities::rr_0574_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0574: Extended: Ring scan lattice modules optimize registry v14");
    let second = relayring::capabilities::rr_0574_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0574: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0574: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0574: stats visits every byte");
}
