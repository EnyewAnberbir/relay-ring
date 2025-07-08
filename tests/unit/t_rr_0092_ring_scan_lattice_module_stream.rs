//! Integration test for `RR-0092` (stream).
//! Ring scan lattice modules harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0092_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5f, 0x61];
    let direct = relayring::capabilities::rr_0092_ring_scan_lattice_module::evaluate(fixture).expect("RR-0092: direct Ring scan lattice modules harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0092_ring_scan_lattice_module::evaluate(&copied).expect("RR-0092: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0092: stream path must consume input");
}
