//! Integration test for `RR-0633` (stream).
//! Extended: Ring scan lattice modules wire planner v73 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0633_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x82];
    let direct = relayring::capabilities::rr_0633_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0633: direct Extended: Ring scan lattice modules wire planner v73");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0633_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0633: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0633: stream path must consume input");
}
