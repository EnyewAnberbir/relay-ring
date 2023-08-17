//! Integration test for `RR-0074` (basic).
//! Ring scan lattice modules optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0074_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let first = relayring::capabilities::rr_0074_ring_scan_lattice_module::evaluate(fixture).expect("RR-0074: Ring scan lattice modules optimize registry v14");
    let second = relayring::capabilities::rr_0074_ring_scan_lattice_module::evaluate(fixture).expect("RR-0074: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0074: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0074: window consumes the whole buffer");
}
