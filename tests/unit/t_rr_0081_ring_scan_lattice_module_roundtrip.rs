//! Integration test for `RR-0081` (roundtrip).
//! Ring scan lattice modules extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0081_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let a = relayring::capabilities::rr_0081_ring_scan_lattice_module::evaluate(fixture).expect("RR-0081 first pass");
    let b = relayring::capabilities::rr_0081_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
