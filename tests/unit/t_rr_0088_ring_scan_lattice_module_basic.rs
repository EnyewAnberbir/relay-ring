//! Integration test for `RR-0088` (basic).
//! Ring scan lattice modules refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0088_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x5d];
    let first = relayring::capabilities::rr_0088_ring_scan_lattice_module::evaluate(fixture).expect("RR-0088: Ring scan lattice modules refactor mutator v28");
    let second = relayring::capabilities::rr_0088_ring_scan_lattice_module::evaluate(fixture).expect("RR-0088: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0088: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0088: stats visits every byte");
}
