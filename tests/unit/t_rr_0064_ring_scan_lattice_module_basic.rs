//! Integration test for `RR-0064` (basic).
//! Ring scan lattice modules optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0064_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let first = relayring::capabilities::rr_0064_ring_scan_lattice_module::evaluate(fixture).expect("RR-0064: Ring scan lattice modules optimize registry v4");
    let second = relayring::capabilities::rr_0064_ring_scan_lattice_module::evaluate(fixture).expect("RR-0064: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0064: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0064: scanner should emit domain hints");
}
