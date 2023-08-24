//! Integration test for `RR-0120` (basic).
//! Ring scan lattice modules implement pipeline v60 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0120_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let first = relayring::capabilities::rr_0120_ring_scan_lattice_module::evaluate(fixture).expect("RR-0120: Ring scan lattice modules implement pipeline v60");
    let second = relayring::capabilities::rr_0120_ring_scan_lattice_module::evaluate(fixture).expect("RR-0120: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0120: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0120: window consumes the whole buffer");
}
