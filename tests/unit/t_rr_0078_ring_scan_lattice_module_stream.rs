//! Integration test for `RR-0078` (stream).
//! Ring scan lattice modules refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0078_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x53];
    let direct = relayring::capabilities::rr_0078_ring_scan_lattice_module::evaluate(fixture).expect("RR-0078: direct Ring scan lattice modules refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0078_ring_scan_lattice_module::evaluate(&copied).expect("RR-0078: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0078: stream path must consume input");
}
