//! Integration test for `RR-0109` (basic).
//! Ring scan lattice modules benchmark reporter v49 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0109_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x72];
    let first = relayring::capabilities::rr_0109_ring_scan_lattice_module::evaluate(fixture).expect("RR-0109: Ring scan lattice modules benchmark reporter v49");
    let second = relayring::capabilities::rr_0109_ring_scan_lattice_module::evaluate(fixture).expect("RR-0109: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0109: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0109: scanner should emit domain hints");
}
