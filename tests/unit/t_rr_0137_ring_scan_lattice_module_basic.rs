//! Integration test for `RR-0137` (basic).
//! Ring scan lattice modules integrate validator v77 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0137_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x8e];
    let first = relayring::capabilities::rr_0137_ring_scan_lattice_module::evaluate(fixture).expect("RR-0137: Ring scan lattice modules integrate validator v77");
    let second = relayring::capabilities::rr_0137_ring_scan_lattice_module::evaluate(fixture).expect("RR-0137: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0137: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0137: stats visits every byte");
}
