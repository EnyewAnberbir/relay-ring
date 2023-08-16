//! Integration test for `RR-0062` (basic).
//! Ring scan lattice modules harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0062_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let first = relayring::capabilities::rr_0062_ring_scan_lattice_module::evaluate(fixture).expect("RR-0062: Ring scan lattice modules harden index v2");
    let second = relayring::capabilities::rr_0062_ring_scan_lattice_module::evaluate(fixture).expect("RR-0062: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0062: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0062: stats visits every byte");
}
