//! Integration test for `RR-0869` (stream).
//! Extended: Gate surfaces seal index optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0869_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x70];
    let direct = relayring::capabilities::rr_0869_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0869: direct Extended: Gate surfaces seal index optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0869_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0869: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0869: stream path must consume input");
}
