//! Integration test for `RR-0095` (stream).
//! Ring scan lattice modules validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0095_ring_scan_lattice_module_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x64];
    let direct = relayring::capabilities::rr_0095_ring_scan_lattice_module::evaluate(fixture).expect("RR-0095: direct Ring scan lattice modules validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0095_ring_scan_lattice_module::evaluate(&copied).expect("RR-0095: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0095: stream path must consume input");
}
