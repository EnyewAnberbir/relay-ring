//! Integration test for `RR-0100` (basic).
//! Ring scan lattice modules implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0100_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x69];
    let first = relayring::capabilities::rr_0100_ring_scan_lattice_module::evaluate(fixture).expect("RR-0100: Ring scan lattice modules implement pipeline v40");
    let second = relayring::capabilities::rr_0100_ring_scan_lattice_module::evaluate(fixture).expect("RR-0100: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0100: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0100: scanner should emit domain hints");
}
