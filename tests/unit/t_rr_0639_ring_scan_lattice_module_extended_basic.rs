//! Integration test for `RR-0639` (basic).
//! Extended: Ring scan lattice modules benchmark reporter v79 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0639_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x88];
    let first = relayring::capabilities::rr_0639_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0639: Extended: Ring scan lattice modules benchmark reporter v79");
    let second = relayring::capabilities::rr_0639_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0639: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0639: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0639: window consumes the whole buffer");
}
