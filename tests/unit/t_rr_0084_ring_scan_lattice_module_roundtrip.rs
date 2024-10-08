//! Integration test for `RR-0084` (roundtrip).
//! Ring scan lattice modules optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0084_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x59];
    let a = relayring::capabilities::rr_0084_ring_scan_lattice_module::evaluate(fixture).expect("RR-0084 first pass");
    let b = relayring::capabilities::rr_0084_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
