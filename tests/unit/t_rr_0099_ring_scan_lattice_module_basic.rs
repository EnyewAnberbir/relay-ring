//! Integration test for `RR-0099` (basic).
//! Ring scan lattice modules benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0099_ring_scan_lattice_module_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x68];
    let first = relayring::capabilities::rr_0099_ring_scan_lattice_module::evaluate(fixture).expect("RR-0099: Ring scan lattice modules benchmark reporter v39");
    let second = relayring::capabilities::rr_0099_ring_scan_lattice_module::evaluate(fixture).expect("RR-0099: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0099: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0099: stats visits every byte");
}
