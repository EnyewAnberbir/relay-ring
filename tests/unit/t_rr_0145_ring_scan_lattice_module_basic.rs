//! Integration test for `RR-0145` (basic).
//! Ring scan lattice modules validate resolver v85 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0145_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96];
    let first = relayring::capabilities::rr_0145_ring_scan_lattice_module::evaluate(fixture).expect("RR-0145: Ring scan lattice modules validate resolver v85");
    let second = relayring::capabilities::rr_0145_ring_scan_lattice_module::evaluate(fixture).expect("RR-0145: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0145: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0145: stats visits every byte");
}
