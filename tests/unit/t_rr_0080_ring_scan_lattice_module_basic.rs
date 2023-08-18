//! Integration test for `RR-0080` (basic).
//! Ring scan lattice modules implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0080_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x55];
    let first = relayring::capabilities::rr_0080_ring_scan_lattice_module::evaluate(fixture).expect("RR-0080: Ring scan lattice modules implement pipeline v20");
    let second = relayring::capabilities::rr_0080_ring_scan_lattice_module::evaluate(fixture).expect("RR-0080: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0080: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0080: window consumes the whole buffer");
}
