//! Integration test for `RR-0102` (stream).
//! Ring scan lattice modules harden index v42 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0102_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6b];
    let direct = relayring::capabilities::rr_0102_ring_scan_lattice_module::evaluate(fixture).expect("RR-0102: direct Ring scan lattice modules harden index v42");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0102_ring_scan_lattice_module::evaluate(&copied).expect("RR-0102: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0102: stream path must consume input");
}
