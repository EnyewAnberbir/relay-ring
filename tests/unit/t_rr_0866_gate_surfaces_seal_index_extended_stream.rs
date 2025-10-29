//! Integration test for `RR-0866` (stream).
//! Extended: Gate surfaces seal index extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0866_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6b, 0x6d];
    let direct = relayring::capabilities::rr_0866_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0866: direct Extended: Gate surfaces seal index extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0866_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0866: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0866: stream path must consume input");
}
