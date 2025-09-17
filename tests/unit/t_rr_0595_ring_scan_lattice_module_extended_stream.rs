//! Integration test for `RR-0595` (stream).
//! Extended: Ring scan lattice modules validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0595_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x5c];
    let direct = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0595: direct Extended: Ring scan lattice modules validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0595_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0595: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0595: stream path must consume input");
}
