//! Integration test for `RR-0068` (basic).
//! Ring scan lattice modules refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0068_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let first = relayring::capabilities::rr_0068_ring_scan_lattice_module::evaluate(fixture).expect("RR-0068: Ring scan lattice modules refactor mutator v8");
    let second = relayring::capabilities::rr_0068_ring_scan_lattice_module::evaluate(fixture).expect("RR-0068: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0068: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0068: stats visits every byte");
}
