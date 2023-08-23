//! Integration test for `RR-0117` (basic).
//! Ring scan lattice modules integrate validator v57 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0117_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x7a];
    let first = relayring::capabilities::rr_0117_ring_scan_lattice_module::evaluate(fixture).expect("RR-0117: Ring scan lattice modules integrate validator v57");
    let second = relayring::capabilities::rr_0117_ring_scan_lattice_module::evaluate(fixture).expect("RR-0117: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0117: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0117: window consumes the whole buffer");
}
