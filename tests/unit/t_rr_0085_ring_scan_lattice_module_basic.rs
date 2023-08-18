//! Integration test for `RR-0085` (basic).
//! Ring scan lattice modules validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0085_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let first = relayring::capabilities::rr_0085_ring_scan_lattice_module::evaluate(fixture).expect("RR-0085: Ring scan lattice modules validate resolver v25");
    let second = relayring::capabilities::rr_0085_ring_scan_lattice_module::evaluate(fixture).expect("RR-0085: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0085: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0085: window consumes the whole buffer");
}
