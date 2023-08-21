//! Integration test for `RR-0089` (basic).
//! Ring scan lattice modules benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0089_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5c, 0x5e];
    let first = relayring::capabilities::rr_0089_ring_scan_lattice_module::evaluate(fixture).expect("RR-0089: Ring scan lattice modules benchmark reporter v29");
    let second = relayring::capabilities::rr_0089_ring_scan_lattice_module::evaluate(fixture).expect("RR-0089: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0089: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0089: window consumes the whole buffer");
}
