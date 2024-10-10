//! Integration test for `RR-0111` (roundtrip).
//! Ring scan lattice modules extend codec v51 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0111_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x72, 0x74];
    let a = relayring::capabilities::rr_0111_ring_scan_lattice_module::evaluate(fixture).expect("RR-0111 first pass");
    let b = relayring::capabilities::rr_0111_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
