//! Integration test for `RR-0081` (basic).
//! Ring scan lattice modules extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0081_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let first = relayring::capabilities::rr_0081_ring_scan_lattice_module::evaluate(fixture).expect("RR-0081: Ring scan lattice modules extend codec v21");
    let second = relayring::capabilities::rr_0081_ring_scan_lattice_module::evaluate(fixture).expect("RR-0081: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0081: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0081: window consumes the whole buffer");
}
