//! Integration test for `RR-0572` (stream).
//! Extended: Ring scan lattice modules harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0572_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let direct = relayring::capabilities::rr_0572_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0572: direct Extended: Ring scan lattice modules harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0572_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0572: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0572: stream path must consume input");
}
