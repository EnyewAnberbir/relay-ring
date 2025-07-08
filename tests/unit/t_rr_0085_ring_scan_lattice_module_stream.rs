//! Integration test for `RR-0085` (stream).
//! Ring scan lattice modules validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0085_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x5a];
    let direct = relayring::capabilities::rr_0085_ring_scan_lattice_module::evaluate(fixture).expect("RR-0085: direct Ring scan lattice modules validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0085_ring_scan_lattice_module::evaluate(&copied).expect("RR-0085: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0085: stream path must consume input");
}
