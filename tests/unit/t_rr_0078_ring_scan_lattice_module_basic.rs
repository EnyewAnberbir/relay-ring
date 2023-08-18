//! Integration test for `RR-0078` (basic).
//! Ring scan lattice modules refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0078_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x53];
    let first = relayring::capabilities::rr_0078_ring_scan_lattice_module::evaluate(fixture).expect("RR-0078: Ring scan lattice modules refactor mutator v18");
    let second = relayring::capabilities::rr_0078_ring_scan_lattice_module::evaluate(fixture).expect("RR-0078: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0078: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0078: stats visits every byte");
}
