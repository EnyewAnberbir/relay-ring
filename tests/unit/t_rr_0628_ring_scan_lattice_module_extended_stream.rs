//! Integration test for `RR-0628` (stream).
//! Extended: Ring scan lattice modules refactor mutator v68 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0628_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let direct = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0628: direct Extended: Ring scan lattice modules refactor mutator v68");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0628_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0628: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0628: stream path must consume input");
}
