//! Integration test for `RR-0607` (stream).
//! Extended: Ring scan lattice modules integrate validator v47 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0607_ring_scan_lattice_module_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x68];
    let direct = relayring::capabilities::rr_0607_ring_scan_lattice_module_extended::evaluate(fixture).expect("RR-0607: direct Extended: Ring scan lattice modules integrate validator v47");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0607_ring_scan_lattice_module_extended::evaluate(&copied).expect("RR-0607: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0607: stream path must consume input");
}
