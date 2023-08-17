//! Integration test for `RR-0070` (basic).
//! Ring scan lattice modules implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0070_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let first = relayring::capabilities::rr_0070_ring_scan_lattice_module::evaluate(fixture).expect("RR-0070: Ring scan lattice modules implement pipeline v10");
    let second = relayring::capabilities::rr_0070_ring_scan_lattice_module::evaluate(fixture).expect("RR-0070: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0070: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0070: stats visits every byte");
}
