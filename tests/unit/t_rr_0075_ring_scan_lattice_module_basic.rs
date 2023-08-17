//! Integration test for `RR-0075` (basic).
//! Ring scan lattice modules validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0075_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let first = relayring::capabilities::rr_0075_ring_scan_lattice_module::evaluate(fixture).expect("RR-0075: Ring scan lattice modules validate resolver v15");
    let second = relayring::capabilities::rr_0075_ring_scan_lattice_module::evaluate(fixture).expect("RR-0075: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0075: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0075: scanner should emit domain hints");
}
