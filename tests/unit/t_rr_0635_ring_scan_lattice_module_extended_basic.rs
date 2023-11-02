//! Integration test for `RR-0635` (basic).
//! Extended: Ring scan lattice modules validate resolver v75 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0635_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x84];
    let first = relayring::capabilities::rr_0635_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0635: Extended: Ring scan lattice modules validate resolver v75");
    let second = relayring::capabilities::rr_0635_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0635: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0635: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0635: stats visits every byte");
}
