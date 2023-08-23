//! Integration test for `RR-0116` (basic).
//! Ring scan lattice modules export adapter v56 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0116_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x79];
    let first = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(fixture).expect("RR-0116: Ring scan lattice modules export adapter v56");
    let second = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(fixture).expect("RR-0116: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0116: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0116: stats visits every byte");
}
