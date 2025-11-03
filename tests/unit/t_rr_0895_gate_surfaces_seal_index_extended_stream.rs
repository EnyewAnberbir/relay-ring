//! Integration test for `RR-0895` (stream).
//! Extended: Gate surfaces seal index implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0895_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let direct = relayring::capabilities::rr_0895_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0895: direct Extended: Gate surfaces seal index implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0895_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0895: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0895: stream path must consume input");
}
