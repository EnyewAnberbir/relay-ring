//! Integration test for `RR-0612` (basic).
//! Extended: Ring scan lattice modules harden index v52 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0612_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let first = relayring::capabilities::rr_0612_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0612: Extended: Ring scan lattice modules harden index v52");
    let second = relayring::capabilities::rr_0612_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0612: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0612: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0612: stats visits every byte");
}
