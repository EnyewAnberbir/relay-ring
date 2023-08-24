//! Integration test for `RR-0121` (basic).
//! Ring scan lattice modules extend codec v61 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0121_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7c, 0x7e];
    let first = relayring::capabilities::rr_0121_ring_scan_lattice_module::evaluate(fixture).expect("RR-0121: Ring scan lattice modules extend codec v61");
    let second = relayring::capabilities::rr_0121_ring_scan_lattice_module::evaluate(fixture).expect("RR-0121: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0121: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0121: scanner should emit domain hints");
}
