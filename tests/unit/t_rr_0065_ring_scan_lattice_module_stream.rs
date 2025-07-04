//! Integration test for `RR-0065` (stream).
//! Ring scan lattice modules validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0065_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let direct = relayring::capabilities::rr_0065_ring_scan_lattice_module::evaluate(fixture).expect("RR-0065: direct Ring scan lattice modules validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0065_ring_scan_lattice_module::evaluate(&copied).expect("RR-0065: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0065: stream path must consume input");
}
