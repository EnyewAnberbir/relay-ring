//! Integration test for `RR-0597` (basic).
//! Extended: Ring scan lattice modules integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0597_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5c, 0x5e];
    let first = relayring::capabilities::rr_0597_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0597: Extended: Ring scan lattice modules integrate validator v37");
    let second = relayring::capabilities::rr_0597_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0597: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0597: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0597: stats visits every byte");
}
