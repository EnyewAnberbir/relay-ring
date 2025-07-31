//! Integration test for `RR-0259` (stream).
//! Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0259_journal_otlp_bridge_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let direct = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(fixture).expect("RR-0259: direct Journal OTLP bridge benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0259_journal_otlp_bridge_benc::evaluate(&copied).expect("RR-0259: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0259: stream path must consume input");
}
