//! Integration test for `RR-0107` (basic).
//! Ring scan lattice modules integrate validator v47 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0107_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x70];
    let first = relayring::capabilities::rr_0107_ring_scan_lattice_module::evaluate(fixture).expect("RR-0107: Ring scan lattice modules integrate validator v47");
    let second = relayring::capabilities::rr_0107_ring_scan_lattice_module::evaluate(fixture).expect("RR-0107: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0107: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0107: window consumes the whole buffer");
}
