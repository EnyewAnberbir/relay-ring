//! Integration test for `RR-0144` (basic).
//! Ring scan lattice modules optimize registry v84 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0144_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x93, 0x95];
    let first = relayring::capabilities::rr_0144_ring_scan_lattice_module::evaluate(fixture).expect("RR-0144: Ring scan lattice modules optimize registry v84");
    let second = relayring::capabilities::rr_0144_ring_scan_lattice_module::evaluate(fixture).expect("RR-0144: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0144: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0144: window consumes the whole buffer");
}
