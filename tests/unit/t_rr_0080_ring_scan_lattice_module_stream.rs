//! Integration test for `RR-0080` (stream).
//! Ring scan lattice modules implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0080_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x55];
    let direct = relayring::capabilities::rr_0080_ring_scan_lattice_module::evaluate(fixture).expect("RR-0080: direct Ring scan lattice modules implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0080_ring_scan_lattice_module::evaluate(&copied).expect("RR-0080: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0080: stream path must consume input");
}
