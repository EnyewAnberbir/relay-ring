//! Integration test for `RR-0106` (stream).
//! Ring scan lattice modules export adapter v46 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0106_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x6f];
    let direct = relayring::capabilities::rr_0106_ring_scan_lattice_module::evaluate(fixture).expect("RR-0106: direct Ring scan lattice modules export adapter v46");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0106_ring_scan_lattice_module::evaluate(&copied).expect("RR-0106: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0106: stream path must consume input");
}
