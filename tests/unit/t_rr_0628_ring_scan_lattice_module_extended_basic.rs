//! Integration test for `RR-0628` (basic).
//! Extended: Ring scan lattice modules refactor mutator v68 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0628_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let first = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0628: Extended: Ring scan lattice modules refactor mutator v68");
    let second = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0628: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0628: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0628: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
