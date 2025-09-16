//! Integration test for `RR-0582` (stream).
//! Extended: Ring scan lattice modules harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0582_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let direct = relayring::capabilities::rr_0582_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0582: direct Extended: Ring scan lattice modules harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0582_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0582: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0582: stream path must consume input");
}
