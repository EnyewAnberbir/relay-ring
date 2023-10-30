//! Integration test for `RR-0595` (basic).
//! Extended: Ring scan lattice modules validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0595_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let first = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0595: Extended: Ring scan lattice modules validate resolver v35");
    let second = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0595: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0595: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0595: stats visits every byte");
}
