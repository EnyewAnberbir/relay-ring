//! Integration test for `RR-0100` (roundtrip).
//! Ring scan lattice modules implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0100_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x69];
    let a = relayring::capabilities::rr_0100_ring_scan_lattice_module::evaluate(fixture).expect("RR-0100 first pass");
    let b = relayring::capabilities::rr_0100_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
