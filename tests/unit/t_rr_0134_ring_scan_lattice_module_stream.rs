//! Integration test for `RR-0134` (stream).
//! Ring scan lattice modules optimize registry v74 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0134_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x8b];
    let direct = relayring::capabilities::rr_0134_ring_scan_lattice_module::evaluate(fixture).expect("RR-0134: direct Ring scan lattice modules optimize registry v74");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0134_ring_scan_lattice_module::evaluate(&copied).expect("RR-0134: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0134: stream path must consume input");
}
