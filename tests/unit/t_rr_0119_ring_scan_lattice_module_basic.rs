//! Integration test for `RR-0119` (basic).
//! Ring scan lattice modules benchmark reporter v59 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0119_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x7c];
    let first = relayring::capabilities::rr_0119_ring_scan_lattice_module::evaluate(fixture).expect("RR-0119: Ring scan lattice modules benchmark reporter v59");
    let second = relayring::capabilities::rr_0119_ring_scan_lattice_module::evaluate(fixture).expect("RR-0119: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0119: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0119: window consumes the whole buffer");
}
