//! Integration test for `RR-0877` (stream).
//! Extended: Gate surfaces seal index harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0877_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x78];
    let direct = relayring::capabilities::rr_0877_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0877: direct Extended: Gate surfaces seal index harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0877_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0877: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0877: stream path must consume input");
}
