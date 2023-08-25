//! Integration test for `RR-0132` (basic).
//! Ring scan lattice modules harden index v72 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0132_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89];
    let first = relayring::capabilities::rr_0132_ring_scan_lattice_module::evaluate(fixture).expect("RR-0132: Ring scan lattice modules harden index v72");
    let second = relayring::capabilities::rr_0132_ring_scan_lattice_module::evaluate(fixture).expect("RR-0132: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0132: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0132: window consumes the whole buffer");
}
