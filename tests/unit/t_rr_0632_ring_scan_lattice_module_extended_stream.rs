//! Integration test for `RR-0632` (stream).
//! Extended: Ring scan lattice modules harden index v72 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0632_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let direct = relayring::capabilities::rr_0632_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0632: direct Extended: Ring scan lattice modules harden index v72");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0632_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0632: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0632: stream path must consume input");
}
