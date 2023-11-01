//! Integration test for `RR-0615` (basic).
//! Extended: Ring scan lattice modules validate resolver v55 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0615_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x70];
    let first = relayring::capabilities::rr_0615_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0615: Extended: Ring scan lattice modules validate resolver v55");
    let second = relayring::capabilities::rr_0615_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0615: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0615: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0615: window consumes the whole buffer");
}
