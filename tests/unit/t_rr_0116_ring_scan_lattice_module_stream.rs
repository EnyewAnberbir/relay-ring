//! Integration test for `RR-0116` (stream).
//! Ring scan lattice modules export adapter v56 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0116_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x79];
    let direct = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(fixture).expect("RR-0116: direct Ring scan lattice modules export adapter v56");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0116_ring_scan_lattice_module::evaluate(&copied).expect("RR-0116: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0116: stream path must consume input");
}
