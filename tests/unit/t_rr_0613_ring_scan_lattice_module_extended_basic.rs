//! Integration test for `RR-0613` (basic).
//! Extended: Ring scan lattice modules wire planner v53 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0613_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x6e];
    let first = relayring::capabilities::rr_0613_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0613: Extended: Ring scan lattice modules wire planner v53");
    let second = relayring::capabilities::rr_0613_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0613: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0613: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0613: window consumes the whole buffer");
}
