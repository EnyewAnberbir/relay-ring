//! Integration test for `RR-0124` (basic).
//! Ring scan lattice modules optimize registry v64 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0124_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let first = relayring::capabilities::rr_0124_ring_scan_lattice_module::evaluate(fixture).expect("RR-0124: Ring scan lattice modules optimize registry v64");
    let second = relayring::capabilities::rr_0124_ring_scan_lattice_module::evaluate(fixture).expect("RR-0124: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0124: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0124: window consumes the whole buffer");
}
