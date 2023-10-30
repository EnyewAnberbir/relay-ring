//! Integration test for `RR-0601` (basic).
//! Extended: Ring scan lattice modules extend codec v41 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0601_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x62];
    let first = relayring::capabilities::rr_0601_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0601: Extended: Ring scan lattice modules extend codec v41");
    let second = relayring::capabilities::rr_0601_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0601: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0601: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0601: stats visits every byte");
}
