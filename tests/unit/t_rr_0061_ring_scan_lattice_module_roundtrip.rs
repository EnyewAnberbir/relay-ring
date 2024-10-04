//! Integration test for `RR-0061` (roundtrip).
//! Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0061_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let a = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(fixture).expect("RR-0061 first pass");
    let b = relayring::capabilities::rr_0061_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
