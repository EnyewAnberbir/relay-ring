//! Integration test for `RR-0609` (basic).
//! Extended: Ring scan lattice modules benchmark reporter v49 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0609_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x6a];
    let first = relayring::capabilities::rr_0609_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0609: Extended: Ring scan lattice modules benchmark reporter v49");
    let second = relayring::capabilities::rr_0609_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0609: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0609: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0609: scanner should emit domain hints");
}
