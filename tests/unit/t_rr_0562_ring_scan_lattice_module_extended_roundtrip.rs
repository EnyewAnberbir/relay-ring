//! Integration test for `RR-0562` (roundtrip).
//! Extended: Ring scan lattice modules harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0562_ring_scan_lattice_module_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let a = relayring::capabilities::rr_0562_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0562 first pass");
    let b = relayring::capabilities::rr_0562_ring_scan_lattice_module_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
