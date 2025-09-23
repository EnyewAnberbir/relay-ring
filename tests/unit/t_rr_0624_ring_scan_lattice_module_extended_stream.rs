//! Integration test for `RR-0624` (stream).
//! Extended: Ring scan lattice modules optimize registry v64 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0624_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0x79];
    let direct = relayring::capabilities::rr_0624_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0624: direct Extended: Ring scan lattice modules optimize registry v64");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0624_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0624: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0624: stream path must consume input");
}
