//! Integration test for `RR-0122` (stream).
//! Ring scan lattice modules harden index v62 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0122_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7d, 0x7f];
    let direct = relayring::capabilities::rr_0122_ring_scan_lattice_module::evaluate(fixture).expect("RR-0122: direct Ring scan lattice modules harden index v62");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0122_ring_scan_lattice_module::evaluate(&copied).expect("RR-0122: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0122: stream path must consume input");
}
