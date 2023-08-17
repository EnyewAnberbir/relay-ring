//! Integration test for `RR-0073` (basic).
//! Ring scan lattice modules wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0073_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let first = relayring::capabilities::rr_0073_ring_scan_lattice_module::evaluate(fixture).expect("RR-0073: Ring scan lattice modules wire planner v13");
    let second = relayring::capabilities::rr_0073_ring_scan_lattice_module::evaluate(fixture).expect("RR-0073: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0073: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0073: window consumes the whole buffer");
}
