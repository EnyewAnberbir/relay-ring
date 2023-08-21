//! Integration test for `RR-0097` (basic).
//! Ring scan lattice modules integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0097_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x66];
    let first = relayring::capabilities::rr_0097_ring_scan_lattice_module::evaluate(fixture).expect("RR-0097: Ring scan lattice modules integrate validator v37");
    let second = relayring::capabilities::rr_0097_ring_scan_lattice_module::evaluate(fixture).expect("RR-0097: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0097: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0097: window consumes the whole buffer");
}
