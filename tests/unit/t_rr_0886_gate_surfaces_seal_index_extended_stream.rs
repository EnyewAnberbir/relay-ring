//! Integration test for `RR-0886` (stream).
//! Extended: Gate surfaces seal index extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0886_gate_surfaces_seal_index_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x81];
    let direct = relayring::capabilities::rr_0886_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0886: direct Extended: Gate surfaces seal index extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0886_gate_surfaces_seal_index_extended::evaluate(&copied).expect("RR-0886: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0886: stream path must consume input");
}
