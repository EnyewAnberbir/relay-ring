//! Integration test for `RR-0063` (basic).
//! Ring scan lattice modules wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0063_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let first = relayring::capabilities::rr_0063_ring_scan_lattice_module::evaluate(fixture).expect("RR-0063: Ring scan lattice modules wire planner v3");
    let second = relayring::capabilities::rr_0063_ring_scan_lattice_module::evaluate(fixture).expect("RR-0063: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0063: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0063: stats visits every byte");
}
