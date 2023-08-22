//! Integration test for `RR-0102` (basic).
//! Ring scan lattice modules harden index v42 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0102_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let first = relayring::capabilities::rr_0102_ring_scan_lattice_module::evaluate(fixture).expect("RR-0102: Ring scan lattice modules harden index v42");
    let second = relayring::capabilities::rr_0102_ring_scan_lattice_module::evaluate(fixture).expect("RR-0102: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0102: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0102: scanner should emit domain hints");
}
