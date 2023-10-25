//! Integration test for `RR-0569` (basic).
//! Extended: Ring scan lattice modules benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0569_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let first = relayring::capabilities::rr_0569_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0569: Extended: Ring scan lattice modules benchmark reporter v9");
    let second = relayring::capabilities::rr_0569_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0569: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0569: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0569: scanner should emit domain hints");
}
