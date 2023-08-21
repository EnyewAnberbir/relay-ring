//! Integration test for `RR-0093` (basic).
//! Ring scan lattice modules wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0093_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x62];
    let first = relayring::capabilities::rr_0093_ring_scan_lattice_module::evaluate(fixture).expect("RR-0093: Ring scan lattice modules wire planner v33");
    let second = relayring::capabilities::rr_0093_ring_scan_lattice_module::evaluate(fixture).expect("RR-0093: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0093: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0093: window consumes the whole buffer");
}
