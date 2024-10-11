//! Integration test for `RR-0125` (roundtrip).
//! Ring scan lattice modules validate resolver v65 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0125_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let a = relayring::capabilities::rr_0125_ring_scan_lattice_module::evaluate(fixture).expect("RR-0125 first pass");
    let b = relayring::capabilities::rr_0125_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
