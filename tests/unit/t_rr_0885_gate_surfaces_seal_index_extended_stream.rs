//! Integration test for `RR-0885` (stream).
//! Extended: Gate surfaces seal index implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0885_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x80];
    let direct = relayring::capabilities::rr_0885_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0885: direct Extended: Gate surfaces seal index implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0885_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0885: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0885: stream path must consume input");
}
