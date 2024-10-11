//! Integration test for `RR-0114` (roundtrip).
//! Ring scan lattice modules optimize registry v54 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0114_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x75, 0x77];
    let a = relayring::capabilities::rr_0114_ring_scan_lattice_module::evaluate(fixture).expect("RR-0114 first pass");
    let b = relayring::capabilities::rr_0114_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
