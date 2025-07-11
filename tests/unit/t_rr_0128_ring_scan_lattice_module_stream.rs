//! Integration test for `RR-0128` (stream).
//! Ring scan lattice modules refactor mutator v68 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0128_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x85];
    let direct = relayring::capabilities::rr_0128_ring_scan_lattice_module::evaluate(fixture).expect("RR-0128: direct Ring scan lattice modules refactor mutator v68");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0128_ring_scan_lattice_module::evaluate(&copied).expect("RR-0128: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0128: stream path must consume input");
}
