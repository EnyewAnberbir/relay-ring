//! Integration test for `RR-0616` (stream).
//! Extended: Ring scan lattice modules export adapter v56 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0616_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x71];
    let direct = relayring::capabilities::rr_0616_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0616: direct Extended: Ring scan lattice modules export adapter v56");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0616_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0616: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0616: stream path must consume input");
}
