//! Integration test for `RR-0136` (basic).
//! Ring scan lattice modules export adapter v76 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0136_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let first = relayring::capabilities::rr_0136_ring_scan_lattice_module::evaluate(fixture).expect("RR-0136: Ring scan lattice modules export adapter v76");
    let second = relayring::capabilities::rr_0136_ring_scan_lattice_module::evaluate(fixture).expect("RR-0136: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0136: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0136: window consumes the whole buffer");
}
