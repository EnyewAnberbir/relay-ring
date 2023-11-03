//! Integration test for `RR-0644` (basic).
//! Extended: Ring scan lattice modules optimize registry v84 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0644_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let first = relayring::capabilities::rr_0644_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0644: Extended: Ring scan lattice modules optimize registry v84");
    let second = relayring::capabilities::rr_0644_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0644: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0644: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0644: stats visits every byte");
}
