//! Integration test for `RR-0115` (basic).
//! Ring scan lattice modules validate resolver v55 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0115_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x78];
    let first = relayring::capabilities::rr_0115_ring_scan_lattice_module::evaluate(fixture).expect("RR-0115: Ring scan lattice modules validate resolver v55");
    let second = relayring::capabilities::rr_0115_ring_scan_lattice_module::evaluate(fixture).expect("RR-0115: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0115: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0115: window consumes the whole buffer");
}
