//! Integration test for `RR-0606` (basic).
//! Extended: Ring scan lattice modules export adapter v46 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0606_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x67];
    let first = relayring::capabilities::rr_0606_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0606: Extended: Ring scan lattice modules export adapter v46");
    let second = relayring::capabilities::rr_0606_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0606: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0606: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0606: window consumes the whole buffer");
}
