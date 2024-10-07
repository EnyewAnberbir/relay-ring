//! Integration test for `RR-0076` (roundtrip).
//! Ring scan lattice modules export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0076_ring_scan_lattice_module_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x51];
    let a = relayring::capabilities::rr_0076_ring_scan_lattice_module::evaluate(fixture).expect("RR-0076 first pass");
    let b = relayring::capabilities::rr_0076_ring_scan_lattice_module::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
