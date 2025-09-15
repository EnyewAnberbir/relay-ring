//! Integration test for `RR-0581` (stream).
//! Extended: Ring scan lattice modules extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0581_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let direct = relayring::capabilities::rr_0581_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0581: direct Extended: Ring scan lattice modules extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0581_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0581: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0581: stream path must consume input");
}
