//! Integration test for `RR-0112` (basic).
//! Ring scan lattice modules harden index v52 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0112_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x75];
    let first = relayring::capabilities::rr_0112_ring_scan_lattice_module::evaluate(fixture).expect("RR-0112: Ring scan lattice modules harden index v52");
    let second = relayring::capabilities::rr_0112_ring_scan_lattice_module::evaluate(fixture).expect("RR-0112: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0112: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0112: window consumes the whole buffer");
}
