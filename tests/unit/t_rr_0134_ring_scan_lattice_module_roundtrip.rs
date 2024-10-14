//! Integration test for `RR-0134` (roundtrip).
//! Ring scan lattice modules optimize registry v74 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0134_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let a = relayring::capabilities::rr_0134_ring_scan_lattice_module::evaluate(fixture).expect("RR-0134 first pass");
    let b = relayring::capabilities::rr_0134_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
