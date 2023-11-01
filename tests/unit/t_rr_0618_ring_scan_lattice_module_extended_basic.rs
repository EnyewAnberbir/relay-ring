//! Integration test for `RR-0618` (basic).
//! Extended: Ring scan lattice modules refactor mutator v58 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0618_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x71, 0x73];
    let first = relayring::capabilities::rr_0618_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0618: Extended: Ring scan lattice modules refactor mutator v58");
    let second = relayring::capabilities::rr_0618_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0618: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0618: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0618: window consumes the whole buffer");
}
