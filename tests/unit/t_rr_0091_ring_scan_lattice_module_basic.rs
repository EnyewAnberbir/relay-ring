//! Integration test for `RR-0091` (basic).
//! Ring scan lattice modules extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0091_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5e, 0x60];
    let first = relayring::capabilities::rr_0091_ring_scan_lattice_module::evaluate(fixture).expect("RR-0091: Ring scan lattice modules extend codec v31");
    let second = relayring::capabilities::rr_0091_ring_scan_lattice_module::evaluate(fixture).expect("RR-0091: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0091: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0091: scanner should emit domain hints");
}
