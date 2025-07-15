//! Integration test for `RR-0145` (stream).
//! Ring scan lattice modules validate resolver v85 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0145_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x96];
    let direct = relayring::capabilities::rr_0145_ring_scan_lattice_module::evaluate(fixture).expect("RR-0145: direct Ring scan lattice modules validate resolver v85");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0145_ring_scan_lattice_module::evaluate(&copied).expect("RR-0145: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0145: stream path must consume input");
}
