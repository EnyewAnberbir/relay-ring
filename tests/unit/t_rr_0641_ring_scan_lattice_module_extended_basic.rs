//! Integration test for `RR-0641` (basic).
//! Extended: Ring scan lattice modules extend codec v81 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0641_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let first = relayring::capabilities::rr_0641_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0641: Extended: Ring scan lattice modules extend codec v81");
    let second = relayring::capabilities::rr_0641_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0641: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0641: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0641: window consumes the whole buffer");
}
