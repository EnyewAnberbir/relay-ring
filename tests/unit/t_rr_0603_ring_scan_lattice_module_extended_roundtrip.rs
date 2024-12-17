//! Integration test for `RR-0603` (roundtrip).
//! Extended: Ring scan lattice modules wire planner v43 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0603_ring_scan_lattice_module_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let a = relayring::capabilities::rr_0603_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0603 first pass");
    let b = relayring::capabilities::rr_0603_ring_scan_lattice_module_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
