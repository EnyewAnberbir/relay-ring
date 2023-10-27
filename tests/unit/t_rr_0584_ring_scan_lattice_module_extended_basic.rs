//! Integration test for `RR-0584` (basic).
//! Extended: Ring scan lattice modules optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0584_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x51];
    let first = relayring::capabilities::rr_0584_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0584: Extended: Ring scan lattice modules optimize registry v24");
    let second = relayring::capabilities::rr_0584_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0584: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0584: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0584: stats visits every byte");
}
