//! Integration test for `RR-0086` (basic).
//! Ring scan lattice modules export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0086_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x5b];
    let first = relayring::capabilities::rr_0086_ring_scan_lattice_module::evaluate(fixture).expect("RR-0086: Ring scan lattice modules export adapter v26");
    let second = relayring::capabilities::rr_0086_ring_scan_lattice_module::evaluate(fixture).expect("RR-0086: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0086: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0086: window consumes the whole buffer");
}
