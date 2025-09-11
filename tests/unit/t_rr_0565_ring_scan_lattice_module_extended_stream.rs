//! Integration test for `RR-0565` (stream).
//! Extended: Ring scan lattice modules validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0565_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let direct = relayring::capabilities::rr_0565_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0565: direct Extended: Ring scan lattice modules validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0565_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0565: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0565: stream path must consume input");
}
