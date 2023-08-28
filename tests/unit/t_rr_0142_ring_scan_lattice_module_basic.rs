//! Integration test for `RR-0142` (basic).
//! Ring scan lattice modules harden index v82 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0142_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let first = relayring::capabilities::rr_0142_ring_scan_lattice_module::evaluate(fixture).expect("RR-0142: Ring scan lattice modules harden index v82");
    let second = relayring::capabilities::rr_0142_ring_scan_lattice_module::evaluate(fixture).expect("RR-0142: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0142: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0142: window consumes the whole buffer");
}
