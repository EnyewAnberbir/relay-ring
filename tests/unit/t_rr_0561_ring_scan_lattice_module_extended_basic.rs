//! Integration test for `RR-0561` (basic).
//! Extended: Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0561_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let first = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0561: Extended: Ring scan lattice modules extend codec v1");
    let second = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0561: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0561: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0561: stats visits every byte");
}
