//! Integration test for `RR-0573` (basic).
//! Extended: Ring scan lattice modules wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0573_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let first = relayring::capabilities::rr_0573_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0573: Extended: Ring scan lattice modules wire planner v13");
    let second = relayring::capabilities::rr_0573_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0573: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0573: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0573: stats visits every byte");
}
