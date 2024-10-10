//! Integration test for `RR-0104` (roundtrip).
//! Ring scan lattice modules optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0104_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let a = relayring::capabilities::rr_0104_ring_scan_lattice_module::evaluate(fixture).expect("RR-0104 first pass");
    let b = relayring::capabilities::rr_0104_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
