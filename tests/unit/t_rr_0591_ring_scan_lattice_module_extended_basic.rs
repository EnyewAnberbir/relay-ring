//! Integration test for `RR-0591` (basic).
//! Extended: Ring scan lattice modules extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0591_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x58];
    let first = relayring::capabilities::rr_0591_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0591: Extended: Ring scan lattice modules extend codec v31");
    let second = relayring::capabilities::rr_0591_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0591: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0591: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0591: stats visits every byte");
}
