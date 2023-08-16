//! Integration test for `RR-0061` (basic).
//! Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0061_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let first = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(fixture).expect("RR-0061: Ring scan lattice modules extend codec v1");
    let second = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(fixture).expect("RR-0061: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0061: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0061: scanner should emit domain hints");
}
