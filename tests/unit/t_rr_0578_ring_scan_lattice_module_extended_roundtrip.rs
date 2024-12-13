//! Integration test for `RR-0578` (roundtrip).
//! Extended: Ring scan lattice modules refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0578_ring_scan_lattice_module_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let a = relayring::capabilities::rr_0578_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0578 first pass");
    let b = relayring::capabilities::rr_0578_ring_scan_lattice_module_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
