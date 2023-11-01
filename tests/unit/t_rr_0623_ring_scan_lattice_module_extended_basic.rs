//! Integration test for `RR-0623` (basic).
//! Extended: Ring scan lattice modules wire planner v63 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0623_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x78];
    let first = relayring::capabilities::rr_0623_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0623: Extended: Ring scan lattice modules wire planner v63");
    let second = relayring::capabilities::rr_0623_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0623: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0623: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0623: stats visits every byte");
}
