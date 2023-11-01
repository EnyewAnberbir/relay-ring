//! Integration test for `RR-0616` (basic).
//! Extended: Ring scan lattice modules export adapter v56 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0616_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let first = relayring::capabilities::rr_0616_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0616: Extended: Ring scan lattice modules export adapter v56");
    let second = relayring::capabilities::rr_0616_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0616: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0616: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0616: stats visits every byte");
}
