//! Integration test for `RR-0101` (basic).
//! Ring scan lattice modules extend codec v41 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0101_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x6a];
    let first = relayring::capabilities::rr_0101_ring_scan_lattice_module::evaluate(fixture).expect("RR-0101: Ring scan lattice modules extend codec v41");
    let second = relayring::capabilities::rr_0101_ring_scan_lattice_module::evaluate(fixture).expect("RR-0101: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0101: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0101: window consumes the whole buffer");
}
