//! Integration test for `RR-0625` (basic).
//! Extended: Ring scan lattice modules validate resolver v65 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0625_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x7a];
    let first = relayring::capabilities::rr_0625_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0625: Extended: Ring scan lattice modules validate resolver v65");
    let second = relayring::capabilities::rr_0625_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0625: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0625: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0625: stats visits every byte");
}
