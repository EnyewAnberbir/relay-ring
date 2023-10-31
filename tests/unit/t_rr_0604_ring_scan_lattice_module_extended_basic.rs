//! Integration test for `RR-0604` (basic).
//! Extended: Ring scan lattice modules optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0604_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x65];
    let first = relayring::capabilities::rr_0604_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0604: Extended: Ring scan lattice modules optimize registry v44");
    let second = relayring::capabilities::rr_0604_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0604: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0604: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0604: stats visits every byte");
}
