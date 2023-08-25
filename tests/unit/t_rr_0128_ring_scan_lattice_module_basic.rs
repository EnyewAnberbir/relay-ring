//! Integration test for `RR-0128` (basic).
//! Ring scan lattice modules refactor mutator v68 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0128_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x85];
    let first = relayring::capabilities::rr_0128_ring_scan_lattice_module::evaluate(fixture).expect("RR-0128: Ring scan lattice modules refactor mutator v68");
    let second = relayring::capabilities::rr_0128_ring_scan_lattice_module::evaluate(fixture).expect("RR-0128: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0128: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0128: stats visits every byte");
}
