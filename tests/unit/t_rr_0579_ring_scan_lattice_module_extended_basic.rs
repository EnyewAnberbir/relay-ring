//! Integration test for `RR-0579` (basic).
//! Extended: Ring scan lattice modules benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0579_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let first = relayring::capabilities::rr_0579_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0579: Extended: Ring scan lattice modules benchmark reporter v19");
    let second = relayring::capabilities::rr_0579_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0579: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0579: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0579: stats visits every byte");
}
