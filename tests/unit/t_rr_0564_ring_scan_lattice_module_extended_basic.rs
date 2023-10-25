//! Integration test for `RR-0564` (basic).
//! Extended: Ring scan lattice modules optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0564_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let first = relayring::capabilities::rr_0564_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0564: Extended: Ring scan lattice modules optimize registry v4");
    let second = relayring::capabilities::rr_0564_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0564: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0564: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0564: scanner should emit domain hints");
}
