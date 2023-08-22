//! Integration test for `RR-0106` (basic).
//! Ring scan lattice modules export adapter v46 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0106_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x6f];
    let first = relayring::capabilities::rr_0106_ring_scan_lattice_module::evaluate(fixture).expect("RR-0106: Ring scan lattice modules export adapter v46");
    let second = relayring::capabilities::rr_0106_ring_scan_lattice_module::evaluate(fixture).expect("RR-0106: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0106: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0106: stats visits every byte");
}
