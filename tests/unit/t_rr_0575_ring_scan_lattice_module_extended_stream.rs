//! Integration test for `RR-0575` (stream).
//! Extended: Ring scan lattice modules validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0575_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let direct = relayring::capabilities::rr_0575_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0575: direct Extended: Ring scan lattice modules validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0575_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0575: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0575: stream path must consume input");
}
