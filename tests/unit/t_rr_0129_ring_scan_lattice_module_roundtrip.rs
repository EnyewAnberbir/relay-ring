//! Integration test for `RR-0129` (roundtrip).
//! Ring scan lattice modules benchmark reporter v69 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0129_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84, 0x86];
    let a = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(fixture).expect("RR-0129 first pass");
    let b = relayring::capabilities::rr_0129_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
