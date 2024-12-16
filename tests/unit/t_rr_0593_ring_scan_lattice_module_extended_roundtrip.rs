//! Integration test for `RR-0593` (roundtrip).
//! Extended: Ring scan lattice modules wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0593_ring_scan_lattice_module_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let a = relayring::capabilities::rr_0593_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0593 first pass");
    let b = relayring::capabilities::rr_0593_ring_scan_lattice_module_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
