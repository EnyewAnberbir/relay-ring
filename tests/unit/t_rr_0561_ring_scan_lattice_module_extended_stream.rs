//! Integration test for `RR-0561` (stream).
//! Extended: Ring scan lattice modules extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0561_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let direct = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0561: direct Extended: Ring scan lattice modules extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0561_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0561: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0561: stream path must consume input");
}
