//! Integration test for `RR-0066` (basic).
//! Ring scan lattice modules export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0066_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let first = relayring::capabilities::rr_0066_ring_scan_lattice_module::evaluate(fixture).expect("RR-0066: Ring scan lattice modules export adapter v6");
    let second = relayring::capabilities::rr_0066_ring_scan_lattice_module::evaluate(fixture).expect("RR-0066: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0066: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0066: window consumes the whole buffer");
}
