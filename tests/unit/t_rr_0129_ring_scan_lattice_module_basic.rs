//! Integration test for `RR-0129` (basic).
//! Ring scan lattice modules benchmark reporter v69 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0129_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let first = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(fixture).expect("RR-0129: Ring scan lattice modules benchmark reporter v69");
    let second = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(fixture).expect("RR-0129: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0129: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0129: scanner should emit domain hints");
}
