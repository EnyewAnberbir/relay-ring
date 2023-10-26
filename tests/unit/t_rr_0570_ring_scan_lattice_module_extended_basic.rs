//! Integration test for `RR-0570` (basic).
//! Extended: Ring scan lattice modules implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0570_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let first = relayring::capabilities::rr_0570_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0570: Extended: Ring scan lattice modules implement pipeline v10");
    let second = relayring::capabilities::rr_0570_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0570: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0570: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0570: window consumes the whole buffer");
}
