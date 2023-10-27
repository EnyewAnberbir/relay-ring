//! Integration test for `RR-0594` (basic).
//! Extended: Ring scan lattice modules optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0594_ring_scan_lattice_module_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x5b];
    let first = relayring::capabilities::rr_0594_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0594: Extended: Ring scan lattice modules optimize registry v34");
    let second = relayring::capabilities::rr_0594_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0594: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0594: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0594: stats visits every byte");
}
