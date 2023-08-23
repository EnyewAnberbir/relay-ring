//! Integration test for `RR-0113` (basic).
//! Ring scan lattice modules wire planner v53 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0113_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x76];
    let first = relayring::capabilities::rr_0113_ring_scan_lattice_module::evaluate(fixture).expect("RR-0113: Ring scan lattice modules wire planner v53");
    let second = relayring::capabilities::rr_0113_ring_scan_lattice_module::evaluate(fixture).expect("RR-0113: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0113: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0113: stats visits every byte");
}
