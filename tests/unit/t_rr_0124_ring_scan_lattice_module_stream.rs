//! Integration test for `RR-0124` (stream).
//! Ring scan lattice modules optimize registry v64 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0124_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let direct = relayring::capabilities::rr_0124_ring_scan_lattice_module::evaluate(fixture).expect("RR-0124: direct Ring scan lattice modules optimize registry v64");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0124_ring_scan_lattice_module::evaluate(&copied).expect("RR-0124: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0124: stream path must consume input");
}
