//! Integration test for `RR-0620` (basic).
//! Extended: Ring scan lattice modules implement pipeline v60 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0620_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x75];
    let first = relayring::capabilities::rr_0620_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0620: Extended: Ring scan lattice modules implement pipeline v60");
    let second = relayring::capabilities::rr_0620_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0620: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0620: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0620: stats visits every byte");
}
