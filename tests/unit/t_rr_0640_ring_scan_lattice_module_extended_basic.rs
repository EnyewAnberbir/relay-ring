//! Integration test for `RR-0640` (basic).
//! Extended: Ring scan lattice modules implement pipeline v80 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0640_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x89];
    let first = relayring::capabilities::rr_0640_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0640: Extended: Ring scan lattice modules implement pipeline v80");
    let second = relayring::capabilities::rr_0640_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0640: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0640: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0640: stats visits every byte");
}
